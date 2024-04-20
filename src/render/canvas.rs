use crate::common::Rectangle;
use crate::model::Color;
use crate::parsers::SimpleXmlWriter;

use std::io::Write;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) enum CanvasItem {
    SvgChunk(String),
    PngImage(Rectangle, Arc<Vec<u8>>),
    GifImage(Rectangle, Arc<Vec<u8>>),
    JpegImage(Rectangle, Arc<Vec<u8>>),
    SvgImage(Rectangle, String, f32, f32),
}

#[derive(Debug)]
pub(crate) struct Canvas {
    items: Vec<CanvasItem>,
    width: f32,
    height: f32,
    bg_color: Color,
}

impl Canvas {
    pub fn new(width: f32, height: f32, bg_color: Color) -> Self {
        Self {
            width,
            height,
            bg_color,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: CanvasItem) {
        self.items.push(item)
    }

    pub fn into_svg(self) -> crate::Result<String> {
        let mut writer = SimpleXmlWriter::new();

        writer.begin("svg");
        writer.attr("xmlns", "http://www.w3.org/2000/svg");
        writer.attr("xmlns:xlink", "http://www.w3.org/1999/xlink");
        writer.attr("width", self.width);
        writer.attr("height", self.height);
        writer.attr("viewBox", &format!("0 0 {} {}", self.width, self.height));

        writer.begin("rect");
        writer.attr("width", self.width);
        writer.attr("height", self.height);
        writer.attr("fill", &self.bg_color);
        writer.end("rect");

        for item in self.items {
            match item {
                CanvasItem::SvgChunk(data) => {
                    writer.text_raw(&data);
                }
                CanvasItem::PngImage(rect, data) => {
                    write_raster_image_to_svg(&rect, "png", &data, &mut writer)
                }
                CanvasItem::GifImage(rect, data) => {
                    write_raster_image_to_svg(&rect, "gif", &data, &mut writer)
                }
                CanvasItem::JpegImage(rect, data) => {
                    write_raster_image_to_svg(&rect, "jpeg", &data, &mut writer)
                }
                CanvasItem::SvgImage(rect, data, width, height) => {
                    use std::fmt::Write;
                    let scale = (rect.width / width).min(rect.height / height);
                    writer.begin("g");
                    writer.attr_buf("transform", |s| {
                        writeln!(s, "translate({}, {}),scale({})", rect.x, rect.y, scale).unwrap();
                    });
                    writer.text_raw(data.as_str());
                    writer.end("g");
                }
            }
        }

        writer.end("svg");
        Ok(writer.into_string())
    }
}

fn write_raster_image_to_svg(
    rect: &Rectangle,
    format: &str,
    data: &[u8],
    xml: &mut SimpleXmlWriter,
) {
    xml.begin("image");
    xml.attr("x", rect.x);
    xml.attr("y", rect.y);
    xml.attr("width", rect.width);
    xml.attr("height", rect.height);
    xml.attr_buf("xlink:href", |s| {
        s.push_str("data:image/");
        s.push_str(format);
        s.push_str(";base64,");
        let mut enc = base64::write::EncoderStringWriter::from_consumer(
            s,
            &base64::engine::general_purpose::STANDARD,
        );
        enc.write_all(data).unwrap();
    });
    xml.end("image");
}
