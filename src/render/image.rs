use crate::common::step_parser::parse_steps_from_label;
use crate::model::{NodeContentImage, Node, NodeContent, SlideDeck, Step, StepValue, SvgImageData, OraImageData, LoadedImageData};
use crate::render::layout::Rectangle;
use crate::render::GlobalResources;
use imagesize::blob_size;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use usvg::TreeParsing;
use usvg::{fontdb, NonZeroRect, TreeTextToPath};
use usvg_tree::{ImageKind, ImageRendering, NodeExt, NodeKind, ViewBox, Visibility};

use crate::NelsieError;


fn prepare_svg_tree_for_step(step: Step, image: &NodeContentImage, svg_data: &SvgImageData, font_db: &fontdb::Database) -> usvg::Tree {
    let mut tree = usvg::Tree::from_data(&svg_data.data, &usvg::Options::default()).expect("SVG Tree build failed");

    if !image.enable_steps || svg_data.id_visibility.is_empty() || step <= image.shift_steps {
        tree.convert_text(font_db);
        return tree
    }
    for (id, visibility) in &svg_data.id_visibility {
        if !visibility.at_step(step - image.shift_steps) {
            if let Some(node) = tree.node_by_id(&id) {
                node.detach();
            }
        }
    }
    tree.convert_text(font_db);
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
    image: &NodeContentImage,
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
    step: Step,
    image: &NodeContentImage,
    rect: &Rectangle,
    svg_node: &usvg::Node,
    font_db: &fontdb::Database,
) {
    if step <= image.shift_steps {
        return;
    }

    match &image.loaded_image.data {
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
            ImageKind::SVG(prepare_svg_tree_for_step(step, image, data, font_db)),
        ),
        LoadedImageData::Ora(data) => {
            render_ora(step, image, data, svg_node, rect, image.loaded_image.width, image.loaded_image.height)
        }
    }
}