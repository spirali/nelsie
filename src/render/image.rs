use crate::model::{LoadedImageData, NodeContentImage, OraImageData, Step, SvgImageData};

use crate::common::Rectangle;

use crate::parsers::step_parser::parse_steps_from_label;
use crate::render::canvas::{Canvas, CanvasItem};

fn render_ora_to_canvas(
    image: &NodeContentImage,
    step: Step,
    ora_data: &OraImageData,
    rect: Rectangle,
    canvas: &mut Canvas,
) {
    let width = image.loaded_image.width;
    let height = image.loaded_image.height;
    let scale = (rect.width / width).min(rect.height / height);

    for layer in &ora_data.layers {
        if !image.enable_steps
            || layer
                .visibility
                .as_ref()
                .map(|v| *v.at_step(&step))
                .unwrap_or(true)
        {
            canvas.add_item(CanvasItem::PngImage(
                Rectangle {
                    x: layer.x * scale + rect.x,
                    y: layer.y * scale + rect.y,
                    width: layer.width * scale,
                    height: layer.height * scale,
                },
                layer.data.clone(),
            ));
        }
    }
}

fn tree_to_svg(tree: &xmltree::Element) -> String {
    let mut s = Vec::<u8>::new();
    tree.write_with_config(
        &mut s,
        xmltree::EmitterConfig {
            write_document_declaration: false,
            ..Default::default()
        },
    )
    .unwrap();
    String::from_utf8(s).unwrap()
}

fn crawl_svg_for_step(nodes: &mut Vec<xmltree::XMLNode>, step: &Step) {
    nodes.retain_mut(|node| match node {
        xmltree::XMLNode::Element(element) => {
            for (key, value) in &element.attributes {
                if key == "label" && value.contains("**") {
                    if let Some(s) = parse_steps_from_label(value, None) {
                        if !s.at_step(step) {
                            return false;
                        }
                    }
                }
            }
            crawl_svg_for_step(&mut element.children, step);
            true
        }
        _ => true,
    })
}

fn prepare_svg_tree_for_step(
    step: Step,
    image: &NodeContentImage,
    svg_data: &SvgImageData,
) -> String {
    if !image.enable_steps {
        return tree_to_svg(&svg_data.tree);
    }
    let mut tree = svg_data.tree.clone();

    crawl_svg_for_step(&mut tree.children, &step);

    tree_to_svg(&tree)
}

pub(crate) fn render_image_to_canvas(
    image: &NodeContentImage,
    step: &Step,
    rect: &Rectangle,
    canvas: &mut Canvas,
) {
    if rect.width <= 0.00001 || rect.height <= 0.00001 {
        return;
    }
    if step <= &Step::from_int(image.shift_steps) {
        return;
    }
    let step = step.subtract_first_index(image.shift_steps);
    let width = image.loaded_image.width;
    let height = image.loaded_image.height;
    let scale = (rect.width / width).min(rect.height / height);
    let target_width = image.loaded_image.width * scale;
    let target_height = image.loaded_image.height * scale;

    let rect = Rectangle::new(
        rect.x + (rect.width - target_width) / 2.0,
        rect.y + (rect.height - target_height) / 2.0,
        target_width,
        target_height,
    );

    match &image.loaded_image.data {
        LoadedImageData::Png(data) => canvas.add_item(CanvasItem::PngImage(rect, data.clone())),
        LoadedImageData::Jpeg(data) => canvas.add_item(CanvasItem::JpegImage(rect, data.clone())),
        LoadedImageData::Svg(svg) => canvas.add_item(CanvasItem::SvgImage(
            rect,
            prepare_svg_tree_for_step(step, image, svg),
            image.loaded_image.width,
            image.loaded_image.height,
        )),
        LoadedImageData::Ora(ora) => render_ora_to_canvas(image, step, ora, rect, canvas),
    }
}
