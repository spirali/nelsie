use crate::common::Rectangle;
use crate::parsers::SimpleXmlWriter;
use crate::render::canvas::{Canvas, CanvasItem};
use std::io::Write;

impl Canvas {
    pub fn into_svg(self) -> crate::Result<String> {
        let mut writer = SimpleXmlWriter::new();

        svg_begin(&mut writer, self.width, self.height);

        writer.begin("rect");
        writer.attr("width", self.width);
        writer.attr("height", self.height);
        writer.attr("fill", self.bg_color);
        writer.end("rect");

        for item in self.items {
            match item {
                CanvasItem::SvgChunk(data) => {
                    writer.text_raw(&data);
                }
                CanvasItem::PngImage(rect, data) => {
                    write_raster_image_to_svg(&rect, "png", &data, &mut writer)
                }
                CanvasItem::JpegImage(rect, data) => {
                    write_raster_image_to_svg(&rect, "jpeg", &data, &mut writer)
                }
                CanvasItem::SvgImage(rect, data, width, height) => {
                    use std::fmt::Write;
                    writer.begin("g");
                    writer.attr_buf("transform", |s| {
                        writeln!(
                            s,
                            "translate({}, {}),scale({}, {})",
                            rect.x,
                            rect.y,
                            rect.width / width,
                            rect.height / height
                        )
                        .unwrap();
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

pub(crate) fn svg_begin(xml: &mut SimpleXmlWriter, width: f32, height: f32) {
    use std::fmt::Write;
    xml.begin("svg");
    xml.attr("xmlns", "http://www.w3.org/2000/svg");
    xml.attr("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xml.attr("width", width);
    xml.attr("height", height);
    xml.attr_buf("viewBox", |s| {
        write!(s, "0 0 {} {}", width, height).unwrap()
    });
}
