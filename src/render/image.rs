use crate::common::step_parser::parse_steps_from_label;
use crate::model::{Image, Node, NodeContent, SlideDeck, Step, StepValue};
use crate::render::layout::Rectangle;
use crate::render::GlobalResources;
use imagesize::blob_size;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use usvg::roxmltree::ExpandedName;
use usvg::TreeParsing;
use usvg::{fontdb, NonZeroRect, TreeTextToPath};
use usvg_tree::{ImageKind, ImageRendering, NodeExt, NodeKind, ViewBox, Visibility};

use crate::NelsieError;

pub(crate) struct SvgImageData {
    tree: usvg::Tree,
    id_visibility: Vec<(String, StepValue<bool>)>,
    n_steps: Step,
}

pub(crate) struct OraLayer {
    visibility: Option<StepValue<bool>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    data: Arc<Vec<u8>>,
}

pub(crate) struct OraImageData {
    layers: Vec<OraLayer>,
    n_steps: Step,
}

pub(crate) enum LoadedImageData {
    Png(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Svg(SvgImageData),
    Ora(OraImageData),
}

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

pub(crate) fn get_image_size(global_res: &GlobalResources, image: &Image) -> (f32, f32) {
    let img = global_res.get_image(&image.filename).unwrap();
    (img.width, img.height)
}

fn prepare_svg_tree_for_step(step: Step, image: &Image, svg_data: &SvgImageData) -> usvg::Tree {
    if !image.enable_steps || svg_data.id_visibility.is_empty() || step <= image.shift_steps {
        return svg_data.tree.clone();
    }
    let mut tree = svg_data.tree.clone();
    tree.root = tree.root.make_deep_copy();
    for (id, visibility) in &svg_data.id_visibility {
        if !visibility.at_step(step - image.shift_steps) {
            if let Some(node) = tree.node_by_id(&id) {
                node.detach();
            }
        }
    }
    tree
}

fn create_image_node(svg_node: &usvg::Node, rect: &Rectangle, kind: ImageKind) {
    if rect.width > 0.00001 && rect.height > 0.00001 {
        let svg_image = usvg::Image {
            id: String::new(),
            transform: Default::default(),
            visibility: Visibility::Visible,
            view_box: ViewBox {
                rect: usvg::Size::from_wh(rect.width, rect.height)
                    .unwrap()
                    .to_non_zero_rect(rect.x, rect.y),
                aspect: Default::default(),
            },
            rendering_mode: ImageRendering::OptimizeQuality,
            kind,
        };
        svg_node.append(usvg::Node::new(NodeKind::Image(svg_image)))
    }
}

fn render_ora(
    step: Step,
    image: &Image,
    ora_data: &OraImageData,
    svg_node: &usvg::Node,
    rect: &Rectangle,
    width: f32,
    height: f32,
) {
    if rect.width <= 0.00001 || rect.height <= 0.00001 {
        return;
    }
    let scale = (rect.width / width).min(rect.height / height);
    for layer in &ora_data.layers {
        if !image.enable_steps
            || layer
                .visibility
                .as_ref()
                .map(|v| *v.at_step(step))
                .unwrap_or(true)
        {
            create_image_node(
                svg_node,
                &Rectangle {
                    x: layer.x * scale + rect.x,
                    y: layer.y * scale + rect.y,
                    width: layer.width * scale,
                    height: layer.height * scale,
                },
                ImageKind::PNG(layer.data.clone()),
            )
        }
    }
}

pub(crate) fn render_image(
    global_res: &GlobalResources,
    step: Step,
    image: &Image,
    rect: &Rectangle,
    svg_node: &usvg::Node,
) {
    let img = global_res.get_image(&image.filename).unwrap();

    if step <= image.shift_steps {
        return;
    }

    match &img.data {
        LoadedImageData::Png(data) => {
            create_image_node(svg_node, rect, ImageKind::PNG(data.clone()))
        }
        LoadedImageData::Gif(data) => {
            create_image_node(svg_node, rect, ImageKind::GIF(data.clone()))
        }
        LoadedImageData::Jpeg(data) => {
            create_image_node(svg_node, rect, ImageKind::JPEG(data.clone()))
        }
        LoadedImageData::Svg(data) => create_image_node(
            svg_node,
            rect,
            ImageKind::SVG(prepare_svg_tree_for_step(step, image, data)),
        ),
        LoadedImageData::Ora(data) => {
            render_ora(step, image, data, svg_node, rect, img.width, img.height)
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

fn load_svg_image(font_db: &fontdb::Database, raw_data: Vec<u8>) -> crate::Result<LoadedImage> {
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

    let mut tree = usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default())?;
    tree.convert_text(font_db);
    Ok(LoadedImage {
        width: tree.size.width(),
        height: tree.size.height(),
        data: LoadedImageData::Svg(SvgImageData {
            tree,
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

fn load_image(font_db: &fontdb::Database, path: &Path) -> crate::Result<LoadedImage> {
    log::debug!("Loading image: {}", path.display());
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    if extension == "svg" {
        let raw_data = std::fs::read(path)?;
        load_svg_image(font_db, raw_data).map_err(|e| {
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

fn n_steps_from_images(node: &Node, loaded_images: &HashMap<PathBuf, LoadedImage>) -> Step {
    let mut n_steps = 1;
    for content in node.content.values() {
        if let Some(NodeContent::Image(image)) = content {
            if !image.enable_steps {
                continue;
            }
            if let Some(loaded_img) = loaded_images.get(&image.filename) {
                n_steps = max(n_steps, image.shift_steps + loaded_img.n_steps());
            }
        }
    }
    n_steps = max(
        n_steps,
        node.child_nodes()
            .map(|child| n_steps_from_images(child, loaded_images))
            .max()
            .unwrap_or(1),
    );
    n_steps
}

pub(crate) fn load_image_in_deck(
    font_db: &fontdb::Database,
    slide_deck: &mut SlideDeck,
) -> crate::Result<HashMap<PathBuf, LoadedImage>> {
    // Collect paths
    let mut paths = HashSet::new();
    for slide in &slide_deck.slides {
        slide.node.collect_image_paths(&mut paths);
    }

    // Load images
    let mut loaded_images = HashMap::new();
    for path in &paths {
        let image = load_image(font_db, path)?;
        assert!(loaded_images.insert(path.to_path_buf(), image).is_none());
    }

    // Update number of steps per slide
    for slide in &mut slide_deck.slides {
        // let mut paths = HashSet::new();
        // slide.node.collect_image_paths(&mut paths);
        //for path in paths.iter() {
        slide.n_steps = max(
            n_steps_from_images(&slide.node, &loaded_images),
            slide.n_steps,
        );
        //}
    }
    Ok(loaded_images)
}

#[cfg(test)]
mod tests {
    // use usvg::{fontdb, TreeParsing};
    //
    // #[test]
    // fn test_xxx() {
    //     let mut opt = usvg::Options::default();
    //     let mut fontdb = fontdb::Database::new();
    //     fontdb.load_system_fonts();
    //
    //     let svg_data = std::fs::read("/home/spirali/projects/nelsie/xxx.svg").unwrap();
    //     let mut tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();
    //     dbg!(tree.root.first_child().unwrap().first_child());
    // }
}
