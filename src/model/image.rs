use crate::model::{StepIndex, StepSet, StepValue};
use crate::parsers::step_parser::parse_steps_from_label;
use imagesize::blob_size;
use resvg::usvg::fontdb;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use svg2pdf::usvg;
use usvg::roxmltree;

use crate::NelsieError;

#[derive(Debug)]
pub(crate) struct SvgImageData {
    /*
       Ideally we would preload SVG tree here, but it cannot be done because usvg:Tree internally
       use Rc, so it is not Send, that makes it is problem for Python binding and potential parallelization
       of rendering.
       On the other hand, it not as bad as it seems at first sight, since we would have to clone
       usvg::Tree for each step because of tree stripping, so recreating from data is not as bad.
    */
    pub tree: xmltree::Element,
    pub steps: StepSet,
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
    pub steps: StepSet,
}

#[derive(Debug)]
pub(crate) enum LoadedImageData {
    Png(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Svg(SvgImageData),
    Ora(OraImageData),
}

#[derive(Debug)]
pub(crate) struct LoadedImage {
    pub image_id: u32,
    pub width: f32,
    pub height: f32,
    pub data: LoadedImageData,
}

impl LoadedImage {
    pub fn update_steps(&self, steps: &mut StepSet, shift_steps: StepIndex) {
        match &self.data {
            LoadedImageData::Png(_) | LoadedImageData::Jpeg(_) => {}
            LoadedImageData::Svg(data) => data.steps.iter().for_each(|s| {
                steps.insert(s.add_first_index(shift_steps));
            }),
            LoadedImageData::Ora(data) => data.steps.iter().for_each(|s| {
                steps.insert(s.add_first_index(shift_steps));
            }),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ImageManager {
    loaded_images: HashMap<PathBuf, Arc<LoadedImage>>,
}

impl ImageManager {
    pub fn load_image(
        &mut self,
        path: &Path,
        font_db: &fontdb::Database,
    ) -> crate::Result<Arc<LoadedImage>> {
        if let Some(img) = self.loaded_images.get(path) {
            Ok(img.clone())
        } else {
            let mut img = load_image(path, font_db)?;
            img.image_id = self.loaded_images.len() as u32;
            let img_ref = Arc::new(img);
            self.loaded_images
                .insert(path.to_path_buf(), img_ref.clone());
            Ok(img_ref)
        }
    }
}

fn load_raster_image(raw_data: Vec<u8>) -> Option<LoadedImage> {
    let size = blob_size(&raw_data).ok()?;
    let data = match imagesize::image_type(&raw_data) {
        Ok(imagesize::ImageType::Png) => LoadedImageData::Png(Arc::new(raw_data)),
        Ok(imagesize::ImageType::Jpeg) => LoadedImageData::Jpeg(Arc::new(raw_data)),
        _ => return None,
    };
    Some(LoadedImage {
        image_id: 0,
        width: size.width as f32,
        height: size.height as f32,
        data,
    })
}

fn load_svg_image(raw_data: Vec<u8>, font_db: &fontdb::Database) -> crate::Result<LoadedImage> {
    let str_data = std::str::from_utf8(&raw_data)
        .map_err(|_e| NelsieError::Generic("Invalid utf-8 data".to_string()))?;

    let tree = xmltree::Element::parse(raw_data.as_slice())
        .map_err(|e| NelsieError::Generic(format!("SVG parsing failed {e}")))?;

    let xml_tree = roxmltree::Document::parse_with_options(
        str_data,
        roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )?;

    // Parse label step definitions
    let mut steps = StepSet::new();
    for node in xml_tree.descendants() {
        if let Some(label) =
            node.attribute(("http://www.inkscape.org/namespaces/inkscape", "label"))
        {
            parse_steps_from_label(label, Some(&mut steps));
        }
    }

    let usvg_tree = usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default(), font_db)?;

    //tree.convert_text(font_db);
    Ok(LoadedImage {
        image_id: 0,
        width: usvg_tree.size().width(),
        height: usvg_tree.size().height(),
        data: LoadedImageData::Svg(SvgImageData { tree, steps }),
    })
}

fn read_archive_file_as_string<R: std::io::Seek + Read>(
    archive: &mut zip::ZipArchive<R>,
    filename: &str,
) -> zip::result::ZipResult<String> {
    Ok(std::io::read_to_string(archive.by_name(filename)?)?)
}

fn option_unpack<T>(value: Option<T>) -> crate::Result<T> {
    value.ok_or_else(|| NelsieError::Generic("Invalid format".to_string()))
}

fn load_ora_stack<R: std::io::Seek + Read>(
    node: &roxmltree::Node,
    archive: &mut zip::ZipArchive<R>,
    layers: &mut Vec<OraLayer>,
    steps: &mut StepSet,
) -> crate::Result<()> {
    for child in node.children() {
        let tag = child.tag_name().name();
        if tag == "layer" {
            if child
                .attribute("visibility")
                .map(|v| v == "hidden")
                .unwrap_or(false)
            {
                continue;
            }
            let visibility =
                parse_steps_from_label(child.attribute("name").unwrap_or(""), Some(steps));
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
            load_ora_stack(&child, archive, layers, steps)?;
        }
    }
    Ok(())
}

fn load_ora_image(path: &Path) -> crate::Result<LoadedImage> {
    let file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    if read_archive_file_as_string(&mut archive, "mimetype")? != "image/openraster" {
        return Err(NelsieError::Generic("Not an ORA format".to_string()));
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
    let mut steps = StepSet::new();
    load_ora_stack(&image, &mut archive, &mut layers, &mut steps)?;
    layers.reverse();
    Ok(LoadedImage {
        image_id: 0,
        width,
        height,
        data: LoadedImageData::Ora(OraImageData { layers, steps }),
    })
}

fn load_image(path: &Path, font_db: &fontdb::Database) -> crate::Result<LoadedImage> {
    log::debug!("Loading image: {}", path.display());
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    if extension == "svg" {
        let raw_data = std::fs::read(path)?;
        load_svg_image(raw_data, font_db).map_err(|e| {
            NelsieError::Generic(format!("Image '{}' load error: {}", path.display(), e))
        })
    } else if extension == "ora" {
        load_ora_image(path).map_err(|e| {
            NelsieError::Generic(format!("Image '{}' load error: {}", path.display(), e))
        })
    } else {
        let raw_data = std::fs::read(path)?;
        load_raster_image(raw_data).ok_or_else(|| {
            NelsieError::Generic(format!("Image '{}' has unknown format", path.display()))
        })
    }
}

#[derive(Debug)]
pub(crate) struct NodeContentImage {
    pub loaded_image: Arc<LoadedImage>,
    pub enable_steps: bool,
    pub shift_steps: StepIndex,
}
