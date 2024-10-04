
use std::fmt::Write;

use crate::common::Rectangle;
use crate::parsers::SimpleXmlWriter;
use crate::render::canvas::{Canvas, CanvasItem};
use crate::render::rtext::RenderedText;

pub(crate) fn render_text_to_canvas(
    rendered_text: &RenderedText,
    rect: &Rectangle,
    canvas: &mut Canvas,
) {
    let mut xml = SimpleXmlWriter::new();
    render_text_to_svg(&mut xml, rendered_text, rect.x, rect.y);
    canvas.add_item(CanvasItem::SvgChunk(xml.into_string()));
}

pub(crate) fn render_text_to_svg(
    xml: &mut SimpleXmlWriter,
    rendered_text: &RenderedText,
    x: f32,
    y: f32,
) {
    xml.begin("g");
    xml.attr_buf("transform", |s| write!(s, "translate({x}, {y})").unwrap());
    for path in rendered_text.paths() {
        path.write_svg(xml);
    }
    xml.end("g")
}
