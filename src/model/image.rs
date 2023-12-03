use crate::common::step_parser::parse_steps_from_label;
use crate::model::{Node, NodeContent, SlideDeck, Step, StepValue};
use crate::render::GlobalResources;
use imagesize::blob_size;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use usvg::TreeParsing;
use usvg::{fontdb, NonZeroRect, TreeTextToPath};
use usvg_tree::{ImageKind, ImageRendering, NodeExt, NodeKind, ViewBox, Visibility};

use crate::NelsieError;

#[derive(Debug)]
pub(crate) struct SvgImageData {
    /*
        Ideally we would preload SVG tree here, but it cannot be done because usvg:Tree internally
        use Rc so it is not Send, that makes it is problem for Python binding and potential paralelization
        of rendering.
        On the other hand, it not as bad as it seems at first sight, since we would have to clone
        usvg::Tree for each step because of tree stripping, so recreating from data is not as bad.
     */
    pub data: Vec<u8>,
    pub id_visibility: Vec<(String, StepValue<bool>)>,
    pub n_steps: Step,
}

#[derive(Debug)]
pub(crate) struct OraLayer {
    pub visibility: Option<StepValue<bool>>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub data: Arc<Vec<u8>>,
}

#[derive(Debug)]
pub(crate) struct OraImageData {
    pub layers: Vec<OraLayer>,
    pub n_steps: Step,
}

#[derive(Debug)]
pub(crate) enum LoadedImageData {
    Png(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Svg(SvgImageData),
    Ora(OraImageData),
}

#[derive(Debug)]
pub(crate) struct LoadedImage {
    pub width: f32,
    pub height: f32,
    pub data: LoadedImageData,
}

impl LoadedImage {
    pub fn n_steps(&self) -> Step {
        match &self.data {
            LoadedImageData::Png(_) | LoadedImageData::Gif(_) | LoadedImageData::Jpeg(_) => 1,
            LoadedImageData::Svg(data) => data.n_steps,
            LoadedImageData::Ora(data) => data.n_steps,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ImageManager {
    loaded_images: HashMap<PathBuf, Arc<LoadedImage>>
}

impl ImageManager {
    pub fn load_image(&mut self, path: &Path) -> crate::Result<Arc<LoadedImage>> {
        if let Some(img) = self.loaded_images.get(path) {
            Ok(img.clone())
        } else {
            let img = Arc::new(load_image(path)?);
            self.loaded_images.insert(path.to_path_buf(), img.clone());
            Ok(img)
        }
    }
}

fn load_raster_image(raw_data: Vec<u8>) -> Option<LoadedImage> {
    let size = imagesize::blob_size(&raw_data).ok()?;
    let image_type = imagesize::image_type(&raw_data);
    let data_arc = Arc::new(raw_data);
    let data = match image_type {
        Ok(imagesize::ImageType::Png) => LoadedImageData::Png(data_arc),
        Ok(imagesize::ImageType::Jpeg) => LoadedImageData::Jpeg(data_arc),
        Ok(imagesize::ImageType::Gif) => LoadedImageData::Gif(data_arc),
        _ => unreachable!(), // This is safe, otherwise it should already fail in blob_size
    };
    Some(LoadedImage {
        width: size.width as f32,
        height: size.height as f32,
        data,
    })
}

fn load_svg_image(raw_data: Vec<u8>) -> crate::Result<LoadedImage> {
    let str_data = std::str::from_utf8(&raw_data)
        .map_err(|e| NelsieError::GenericError("Invalid utf-8 data".to_string()))?;
    let xml_tree = roxmltree::Document::parse(&str_data)?;

    // Parse label step definitions
    let mut n_steps = 1;
    let mut id_visibility = Vec::new();
    for node in xml_tree.descendants() {
        if let Some(label) =
            node.attribute(("http://www.inkscape.org/namespaces/inkscape", "label"))
        {
            let (steps, n) = if let Some(v) = parse_steps_from_label(&label) {
                v
            } else {
                continue;
            };
            let id = if let Some(id) = node.attribute("id") {
                id
            } else {
                continue;
            };
            n_steps = max(n_steps, n);
            id_visibility.push((id.to_string(), steps));
        }
    }
    // TODO: We need to load tree only for width and hight, so we probably do not need to load whole tree
    let mut tree = usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default())?;
    //tree.convert_text(font_db);
    Ok(LoadedImage {
        width: tree.size.width(),
        height: tree.size.height(),
        data: LoadedImageData::Svg(SvgImageData {
            data: raw_data,
            n_steps,
            id_visibility,
        }),
    })
}

fn read_archive_file_as_string<R: std::io::Seek + std::io::Read>(
    archive: &mut zip::ZipArchive<R>,
    filename: &str,
) -> zip::result::ZipResult<String> {
    Ok(std::io::read_to_string(archive.by_name(filename)?)?)
}

fn option_unpack<T>(value: Option<T>) -> crate::Result<T> {
    value.ok_or_else(|| NelsieError::GenericError("Invalid format".to_string()))
}

fn load_ora_stack<R: std::io::Seek + std::io::Read>(
    node: &roxmltree::Node,
    archive: &mut zip::ZipArchive<R>,
    layers: &mut Vec<OraLayer>,
    n_steps: &mut Step,
) -> crate::Result<()> {
    for child in node.children() {
        let tag = child.tag_name().name();
        if tag == "layer" {
            let visibility =
                parse_steps_from_label(child.attribute("name").unwrap_or("")).map(|(v, n)| {
                    *n_steps = max(*n_steps, n);
                    v
                });
            let src = option_unpack(child.attribute("src"))?;
            let mut file = archive.by_name(src)?;
            let mut image_data = Vec::new();
            file.read_to_end(&mut image_data)?;
            let (width, height) = blob_size(&image_data)
                .map(|sz| (sz.width as f32, sz.height as f32))
                .unwrap_or((0.0, 0.0));
            layers.push(OraLayer {
                visibility,
                x: child
                    .attribute("x")
                    .and_then(|v| str::parse(v).ok())
                    .unwrap_or(0.0),
                y: child
                    .attribute("y")
                    .and_then(|v| str::parse(v).ok())
                    .unwrap_or(0.0),
                width,
                height,
                data: Arc::new(image_data),
            });
        } else if tag == "stack" {
            load_ora_stack(&child, archive, layers, n_steps)?;
        }
    }
    Ok(())
}

fn load_ora_image(path: &Path) -> crate::Result<LoadedImage> {
    let file = File::open(&path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    if read_archive_file_as_string(&mut archive, "mimetype")? != "image/openraster" {
        return Err(NelsieError::GenericError("Not an ORA format".to_string()));
    }
    let stack_data = read_archive_file_as_string(&mut archive, "stack.xml")?;
    let stack_doc = roxmltree::Document::parse(&stack_data)?;
    let image = option_unpack(stack_doc.root().first_child())?;
    let width: f32 = image
        .attribute("w")
        .and_then(|v| str::parse(v).ok())
        .unwrap_or(0.0);
    let height: f32 = image
        .attribute("h")
        .and_then(|v| str::parse(v).ok())
        .unwrap_or(0.0);

    let mut layers = Vec::new();
    let mut n_steps = 1;
    load_ora_stack(&image, &mut archive, &mut layers, &mut n_steps)?;
    layers.reverse();
    Ok(LoadedImage {
        width,
        height,
        data: LoadedImageData::Ora(OraImageData { layers, n_steps }),
    })
}

fn load_image(path: &Path) -> crate::Result<LoadedImage> {
    log::debug!("Loading image: {}", path.display());
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    if extension == "svg" {
        let raw_data = std::fs::read(path)?;
        load_svg_image(raw_data).map_err(|e| {
            NelsieError::GenericError(format!("Image '{}' load error: {}", path.display(), e))
        })
    } else if extension == "ora" {
        load_ora_image(path).map_err(|e| {
            NelsieError::GenericError(format!("Image '{}' load error: {}", path.display(), e))
        })
    } else {
        let raw_data = std::fs::read(path)?;
        load_raster_image(raw_data).ok_or_else(|| {
            NelsieError::GenericError(format!("Image '{}' has unknown format", path.display()))
        })
    }
}

// fn n_steps_from_images(node: &Node, loaded_images: &HashMap<PathBuf, LoadedImage>) -> Step {
//     let mut n_steps = 1;
//     for content in node.content.values() {
//         if let Some(NodeContent::Image(image)) = content {
//             if !image.enable_steps {
//                 continue;
//             }
//             if let Some(loaded_img) = loaded_images.get(&image.filename) {
//                 n_steps = max(n_steps, image.shift_steps + loaded_img.n_steps());
//             }
//         }
//     }
//     n_steps = max(
//         n_steps,
//         node.child_nodes()
//             .map(|child| n_steps_from_images(child, loaded_images))
//             .max()
//             .unwrap_or(1),
//     );
//     n_steps
// }

#[derive(Debug)]
pub(crate) struct NodeContentImage {
    pub loaded_image: Arc<LoadedImage>,
    pub enable_steps: bool,
    pub shift_steps: Step,
}
