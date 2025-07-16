use crate::common::{DrawItem, Rectangle};
use crate::model::LoadedImageData;
use crate::parsers::SimpleXmlWriter;
use crate::render::canvas::{Canvas, CanvasItem};
use crate::render::svgpath::{svg_ellipse, svg_path, svg_rect};
use crate::render::text::RenderedText;
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
                CanvasItem::PngImage { rect, data } => {
                    write_raster_image_to_svg(&rect, "png", &data, &mut writer)
                }
                CanvasItem::JpegImage { rect, data } => {
                    write_raster_image_to_svg(&rect, "jpeg", &data, &mut writer)
                }
                CanvasItem::SvgImage {
                    rect,
                    data,
                    width,
                    height,
                } => render_svg_image_into_svg(&mut writer, data.as_str(), &rect, width, height),
                CanvasItem::Text { text, x, y } => render_text_into_svg(&mut writer, &text, x, y),
                CanvasItem::DrawItems(items) => items
                    .iter()
                    .for_each(|item| write_draw_item_to_svg(&mut writer, item)),
                CanvasItem::Video { rect, video } => {
                    if let Some(image) = &video.cover_image {
                        match &image.data {
                            LoadedImageData::Png(data) => {
                                write_raster_image_to_svg(&rect, "png", data, &mut writer)
                            }
                            LoadedImageData::Jpeg(data) => {
                                write_raster_image_to_svg(&rect, "jpeg", data, &mut writer)
                            }
                            LoadedImageData::Svg(_) | LoadedImageData::Ora(_) => unreachable!(),
                        }
                    }
                }
            }
        }

        writer.end("svg");
        Ok(writer.into_string())
    }
}

fn write_draw_item_to_svg(xml: &mut SimpleXmlWriter, item: &DrawItem) {
    match item {
        DrawItem::Rect(rect) => svg_rect(xml, rect),
        DrawItem::Oval(rect) => svg_ellipse(xml, rect),
        DrawItem::Path(path) => svg_path(xml, path),
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
    xml.attr_buf("viewBox", |s| write!(s, "0 0 {width} {height}").unwrap());
}

fn render_text_into_svg(
    writer: &mut SimpleXmlWriter,
    rendered_text: &RenderedText,
    x: f32,
    y: f32,
) {
    use std::fmt::Write;
    writer.begin("g");
    writer.attr_buf("transform", |s| write!(s, "translate({x}, {y})").unwrap());
    for path in rendered_text.paths() {
        svg_path(writer, path);
    }
    writer.end("g")
}

fn render_svg_image_into_svg(
    writer: &mut SimpleXmlWriter,
    data: &str,
    rect: &Rectangle,
    width: f32,
    height: f32,
) {
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
    writer.text_raw(data);
    writer.end("g");
}
