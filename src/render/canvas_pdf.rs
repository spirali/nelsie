use crate::common::Rectangle;
use crate::model::Resources;
use crate::parsers::SimpleXmlWriter;
use crate::render::canvas::{Canvas, CanvasItem};
use crate::render::canvas_svg::svg_begin;

use crate::render::pdf::{PdfPage, PdfPageElement};
use by_address::ByAddress;
use pdf_writer::Ref;
use std::collections::HashMap;
use std::sync::Arc;
use svg2pdf::usvg;

pub(crate) type PdfImageCache = HashMap<ByAddress<Arc<Vec<u8>>>, Ref>;

impl Canvas {
    pub fn into_pdf_page(
        self,
        resources: &Resources,
        cache: &PdfImageCache,
    ) -> crate::Result<PdfPage> {
        let mut elements: Vec<PdfPageElement> = Vec::with_capacity(self.items.len());

        let close_xml =
            |xml_writer: &mut Option<SimpleXmlWriter>, result: &mut Vec<_>| -> crate::Result<()> {
                if let Some(mut xml) = std::mem::take(xml_writer) {
                    xml.end("svg");
                    let tree = usvg::Tree::from_str(
                        &xml.into_string(),
                        &usvg::Options::default(),
                        &resources.font_db,
                    )?;
                    let (svg_chunk, svg_id) = svg2pdf::to_chunk(
                        &tree,
                        svg2pdf::ConversionOptions::default(),
                        &resources.font_db,
                    );
                    result.push(PdfPageElement::LocalRef(
                        Rectangle::new(0.0, 0.0, self.width, self.height),
                        svg_chunk,
                        svg_id,
                    ));
                }
                Ok(())
            };

        let mut xml_writer: Option<SimpleXmlWriter> = None;
        for item in self.items {
            match item {
                CanvasItem::SvgChunk(svg_chunk) => {
                    if xml_writer.is_none() {
                        let mut xml = SimpleXmlWriter::new();
                        svg_begin(&mut xml, self.width, self.height);
                        xml_writer = Some(xml)
                    };
                    let xml = xml_writer.as_mut().unwrap();
                    xml.text_raw(&svg_chunk);
                }
                CanvasItem::PngImage(rect, data) => {
                    close_xml(&mut xml_writer, &mut elements)?;
                    elements.push(PdfPageElement::GlobalRef(
                        rect,
                        *cache.get(&ByAddress(data)).unwrap(),
                    ));
                }
                CanvasItem::JpegImage(rect, data) => {
                    close_xml(&mut xml_writer, &mut elements)?;
                    elements.push(PdfPageElement::GlobalRef(
                        rect,
                        *cache.get(&ByAddress(data)).unwrap(),
                    ));
                }
                CanvasItem::SvgImage(rect, svg_data, _, _) => {
                    close_xml(&mut xml_writer, &mut elements)?;
                    let tree = usvg::Tree::from_str(
                        &svg_data,
                        &usvg::Options::default(),
                        &resources.font_db,
                    )?;
                    let (svg_chunk, svg_id) = svg2pdf::to_chunk(
                        &tree,
                        svg2pdf::ConversionOptions::default(),
                        &resources.font_db,
                    );
                    elements.push(PdfPageElement::LocalRef(rect, svg_chunk, svg_id));
                }
            }
        }
        close_xml(&mut xml_writer, &mut elements)?;
        Ok(PdfPage {
            elements,
            width: self.width,
            height: self.height,
            bg_color: self.bg_color,
            links: self.links,
        })
    }
}
