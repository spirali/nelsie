use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use usvg::{fontdb, NonZeroRect, TreeTextToPath};
use usvg::roxmltree::ExpandedName;
use usvg_tree::{ImageKind, ImageRendering, NodeKind, ViewBox, Visibility};
use crate::model::{Image, SlideDeck};
use crate::render::GlobalResources;
use crate::render::layout::Rectangle;
use usvg::TreeParsing;


use crate::NelsieError;

pub(crate) struct SvgImageData {
    tree: usvg::Tree,
}

pub(crate) enum LoadedImageData {
    Png(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
    Svg(SvgImageData),
}

pub(crate) struct LoadedImage {
    pub width: f32,
    pub height: f32,
    pub data: LoadedImageData,
}


pub(crate) fn get_image_size(global_res: &GlobalResources, image: &Image) -> (f32, f32) {
    let img = global_res.get_image(&image.filename).unwrap();
    (img.width, img.height)
}

pub(crate) fn render_image(global_res: &GlobalResources, image: &Image, rect: &Rectangle, svg_node: &usvg::Node) {
    let img = global_res.get_image(&image.filename).unwrap();

    let svg_image = usvg::Image {
        id: String::new(),
        transform: Default::default(),
        visibility: Visibility::Visible,
        view_box: ViewBox {
            rect: usvg::Size::from_wh(rect.width, rect.height).unwrap().to_non_zero_rect(rect.x, rect.y),
            aspect: Default::default(),
        },
        rendering_mode: ImageRendering::OptimizeQuality,
        kind: match &img.data {
            LoadedImageData::Png(data) => ImageKind::PNG(data.clone()),
            LoadedImageData::Gif(data) => ImageKind::GIF(data.clone()),
            LoadedImageData::Jpeg(data) => ImageKind::JPEG(data.clone()),
            LoadedImageData::Svg(data) => ImageKind::SVG(data.tree.clone()),
        },
    };
    svg_node.append(usvg::Node::new(NodeKind::Image(svg_image)))
}

fn load_raster_image(raw_data: Vec<u8>) -> Option<LoadedImage> {
    let size = imagesize::blob_size(&raw_data).ok()?;
    let image_type = imagesize::image_type(&raw_data);
    let data_arc = Arc::new(raw_data);
    let data = match image_type {
        Ok(imagesize::ImageType::Png) => LoadedImageData::Png(data_arc),
        Ok(imagesize::ImageType::Jpeg) => LoadedImageData::Jpeg(data_arc),
        Ok(imagesize::ImageType::Gif) => LoadedImageData::Gif(data_arc),
        _ => unreachable!() // This is safe, otherwise it should already fail in blob_size
    };
    Some(LoadedImage {
        width: size.width as f32,
        height: size.height as f32,
        data,
    })
}

fn load_svg_image(font_db: &fontdb::Database, raw_data: Vec<u8>) -> crate::Result<LoadedImage> {
    let str_data = std::str::from_utf8(&raw_data).map_err(|e| NelsieError::GenericError("Invalid utf-8 data".to_string()))?;
    let xml_tree = roxmltree::Document::parse(&str_data)?;
    // xml_tree.descendants().for_each(|node| {
    //     node.attributes();
    //     println!("{:?}", node.attribute("id"));
    //     println!("{:?}", node.attribute(("http://www.inkscape.org/namespaces/inkscape", "label")));
    // });
    let mut tree = usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default())?;
    tree.convert_text(font_db);
    Ok(LoadedImage {
        width: tree.size.width(),
        height: tree.size.height(),
        data: LoadedImageData::Svg(SvgImageData {
            tree
        }),
    })
}

fn load_image(font_db: &fontdb::Database, path: &Path) -> crate::Result<LoadedImage> {
    log::debug!("Loading image: {}", path.display());
    let raw_data = std::fs::read(path)?;
    println!("{} {}", path.display(), path.ends_with(".svg"));
    let extension = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
    if extension == "svg" {
        load_svg_image(font_db, raw_data).map_err(|e| NelsieError::GenericError(format!("Image '{}' load error: {}", path.display(), e)))
    } else {
        load_raster_image(raw_data).ok_or_else(|| NelsieError::GenericError(format!("Image '{}' has unknown format", path.display())))
    }
}

pub(crate) fn load_image_in_deck(font_db: &fontdb::Database, slide_deck: &SlideDeck) -> crate::Result<HashMap<PathBuf, LoadedImage>> {
    let mut paths = HashSet::new();
    for slide in &slide_deck.slides {
        slide.node.collect_image_paths(&mut paths);
    }
    let mut loaded_images = HashMap::new();
    for path in &paths {
        let image = load_image(font_db, path)?;
        assert!(loaded_images.insert(path.to_path_buf(), image).is_none());
    }
    Ok(loaded_images)
}


#[cfg(test)]
mod tests {
    // use usvg::{fontdb, TreeParsing};

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
