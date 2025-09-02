use crate::render::canvas::{Canvas, CanvasItem};
use crate::render::content::{ContentBody, ContentMap};
use crate::render::draw::DrawItem;
use crate::render::svgpath::{svg_ellipse, svg_path, svg_rect};
use crate::render::text::RenderedText;
use crate::utils::sxml::SimpleXmlWriter;
use crate::{ContentId, InMemoryBinImage, Rectangle};
use std::io::Write;

impl Canvas {
    pub fn as_svg(&self, content_map: &ContentMap) -> crate::Result<String> {
        let mut writer = SimpleXmlWriter::new();

        svg_begin(&mut writer, self.width, self.height);

        writer.begin("rect");
        writer.attr("width", self.width);
        writer.attr("height", self.height);
        writer.attr("fill", self.bg_color);
        writer.end("rect");

        for item in self.items() {
            match item {
                CanvasItem::Content { rect, content_id } => {
                    render_content_to_svg(&mut writer, content_map, rect, *content_id);
                }
                CanvasItem::DrawItem(item) => write_draw_item_to_svg(&mut writer, item),
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
    xml.attr("preserveAspectRatio", "none");
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

fn write_g_transform(writer: &mut SimpleXmlWriter, rect: &Rectangle, width: f32, height: f32) {
    use std::fmt::Write;
    let scale_x = rect.width / width;
    let scale_y = rect.height / height;
    writer.begin("g");
    writer.attr_buf("transform", |s| {
        write!(s, "translate({},{})", rect.x, rect.y).unwrap();
        if !(0.999999..=1.000001).contains(&scale_x) || !(0.999999..=1.000001).contains(&scale_y) {
            write!(s, ",scale({},{})", scale_x, scale_y).unwrap()
        }
    });
}

fn render_text_into_svg(
    writer: &mut SimpleXmlWriter,
    rendered_text: &RenderedText,
    rect: &Rectangle,
    width: f32,
    height: f32,
) {
    write_g_transform(writer, rect, width, height);
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
    write_g_transform(writer, rect, width, height);
    writer.text_raw(data);
    writer.end("g");
}

fn render_content_to_svg(
    writer: &mut SimpleXmlWriter,
    content_map: &ContentMap,
    rect: &Rectangle,
    content_id: ContentId,
) {
    let content = content_map.get(&content_id).unwrap();
    let (width, height) = content.size();
    let rect = rect.fit_content_with_aspect_ratio(width, height);
    match content.body() {
        ContentBody::Text((text, _is_shared)) => {
            render_text_into_svg(writer, text, &rect, width, height);
        }
        ContentBody::BinImage(image) => {
            let (format, data) = match image {
                InMemoryBinImage::Png(data) => ("png", data),
                InMemoryBinImage::Jpeg(data) => ("jpeg", data),
            };
            write_raster_image_to_svg(&rect, format, data, writer);
        }
        ContentBody::SvgImage(image) => {
            render_svg_image_into_svg(writer, &image.as_string(), &rect, width, height);
        }
        ContentBody::Composition(items) => {
            write_g_transform(writer, &rect, width, height);
            for (rect, content_id) in items {
                render_content_to_svg(writer, content_map, rect, *content_id);
            }
            writer.end("g")
        }
    }
}
