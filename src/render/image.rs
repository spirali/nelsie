use crate::model::{
    LoadedImageData, NodeContentImage, OraImageData, Step, StepValue, SvgImageData,
};
use std::collections::HashMap;

use crate::render::layout::Rectangle;

use svg2pdf::usvg;
use svg2pdf::usvg::{
    fontdb, Group, ImageKind, ImageRendering, PostProcessingSteps, TreeParsing, TreePostProc,
    ViewBox, Visibility,
};

fn remove_tree_nodes(
    root: &mut Group,
    id_visibility: &HashMap<String, StepValue<bool>>,
    step: Step,
) {
    root.children.retain_mut(|node| {
        if let usvg::Node::Group(ref mut g) = node {
            remove_tree_nodes(g, id_visibility, step);
        }
        id_visibility
            .get(node.id())
            .map(|vis| *vis.at_step(step))
            .unwrap_or(true)
    })
}

fn prepare_svg_tree_for_step(
    step: Step,
    image: &NodeContentImage,
    svg_data: &SvgImageData,
    font_db: &fontdb::Database,
) -> usvg::Tree {
    let mut tree = usvg::Tree::from_data(&svg_data.data, &usvg::Options::default())
        .expect("SVG Tree build failed");

    let postprocessing = PostProcessingSteps {
        convert_text_into_paths: true,
    };

    if !image.enable_steps || svg_data.id_visibility.is_empty() || step <= image.shift_steps {
        tree.postprocess(postprocessing, font_db);
        return tree;
    }
    remove_tree_nodes(
        &mut tree.root,
        &svg_data.id_visibility,
        step - image.shift_steps,
    );
    tree.postprocess(postprocessing, font_db);
    tree
}

fn create_image_node(svg_node: &mut usvg::Group, rect: &Rectangle, kind: ImageKind) {
    if rect.width > 0.00001 && rect.height > 0.00001 {
        let svg_image = usvg::Image {
            id: String::new(),
            visibility: Visibility::Visible,
            view_box: ViewBox {
                rect: usvg::Size::from_wh(rect.width, rect.height)
                    .unwrap()
                    .to_non_zero_rect(rect.x, rect.y),
                aspect: Default::default(),
            },
            rendering_mode: ImageRendering::OptimizeQuality,
            kind,
            abs_transform: Default::default(),
            bounding_box: None,
        };
        svg_node
            .children
            .push(usvg::Node::Image(Box::new(svg_image)))
    }
}

fn render_ora(
    step: Step,
    image: &NodeContentImage,
    ora_data: &OraImageData,
    svg_node: &mut usvg::Group,
    rect: &Rectangle,
    width: f32,
    height: f32,
) {
    if rect.width <= 0.00001 || rect.height <= 0.00001 {
        return;
    }
    if step <= image.shift_steps {
        return;
    }
    let scale = (rect.width / width).min(rect.height / height);
    for layer in &ora_data.layers {
        if !image.enable_steps
            || layer
                .visibility
                .as_ref()
                .map(|v| *v.at_step(step - image.shift_steps))
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
    svg_node: &mut usvg::Group,
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
        LoadedImageData::Ora(data) => render_ora(
            step,
            image,
            data,
            svg_node,
            rect,
            image.loaded_image.width,
            image.loaded_image.height,
        ),
    }
}
