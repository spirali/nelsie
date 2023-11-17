use usvg::NonZeroRect;
use usvg_tree::{ImageKind, ImageRendering, NodeKind, ViewBox, Visibility};
use crate::model::Image;
use crate::render::GlobalResources;
use crate::render::globals::LoadedImageData;
use crate::render::layout::Rectangle;

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
            LoadedImageData::Jpeg(data) => ImageKind::JPEG(data.clone())
        },
    };
    svg_node.append(usvg::Node::new(NodeKind::Image(svg_image)))
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
