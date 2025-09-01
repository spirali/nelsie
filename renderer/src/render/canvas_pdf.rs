use crate::render::canvas::{Canvas, CanvasItem};

use crate::render::composer_pdf::PdfRefAllocator;
use crate::render::content::{ContentBody, ContentMap};
use crate::render::pdfdraw::{PdfWriter, annotations_to_pdf, draw_item_to_pdf, path_to_pdf};
use crate::{ContentId, Rectangle};
use pdf_writer::{Chunk, Filter, Finish, Name, Rect, Ref};
use std::collections::HashMap;

impl Canvas {
    pub fn into_pdf_page(
        self,
        ref_allocator: &PdfRefAllocator,
        page_ref: Ref,
        page_tree_ref: Ref,
        compression_level: u8,
        content_map: &ContentMap,
        content_to_ref: &HashMap<ContentId, Ref>,
    ) -> crate::Result<Chunk> {
        let mut pdf_writer = PdfWriter::new(ref_allocator);
        pdf_writer.content.save_state();
        let [r, g, b] = self.bg_color.as_f32s();
        pdf_writer.content.set_fill_rgb(r, g, b);
        pdf_writer.content.rect(0.0, 0.0, self.width, self.height);
        pdf_writer.content.fill_nonzero();
        pdf_writer.content.restore_state();

        let mut annotation_ids = Vec::new();

        pdf_writer.content.save_state();
        pdf_writer
            .content
            .transform([1.0, 0.0, 0.0, -1.0, 0.0, self.height]);

        for item in self.items() {
            match item {
                CanvasItem::DrawItem(item) => {
                    draw_item_to_pdf(&mut pdf_writer, item);
                }
                CanvasItem::Content { rect, content_id } => {
                    content_into_pdf(
                        &mut pdf_writer,
                        content_map,
                        content_to_ref,
                        rect,
                        *content_id,
                    );
                }
            }
        }
        pdf_writer.content.restore_state();

        annotations_to_pdf(
            &mut pdf_writer,
            self.links,
            self.height,
            &mut annotation_ids,
        );

        let content_ref = pdf_writer.alloc_ref.bump();
        let mut page = pdf_writer.chunk.page(page_ref);
        page.media_box(Rect::new(0.0, 0.0, self.width, self.height));
        page.parent(page_tree_ref);
        page.contents(content_ref);
        if !annotation_ids.is_empty() {
            page.annotations(annotation_ids);
        }
        let mut resources = page.resources();
        let mut objects = resources.x_objects();
        for (name, rf) in pdf_writer.xo_resources {
            objects.pair(Name(name.as_bytes()), rf);
        }
        objects.finish();
        let mut g_states = resources.ext_g_states();
        for (name, rf) in pdf_writer.gs_resources.values() {
            g_states.pair(Name(name.as_bytes()), rf);
        }
        g_states.finish();
        resources.finish();
        page.finish();

        let mut content_data = pdf_writer.content.finish();
        if compression_level > 0 {
            content_data =
                miniz_oxide::deflate::compress_to_vec_zlib(&content_data, compression_level)
        }
        let mut stream = pdf_writer.chunk.stream(content_ref, &content_data);
        if compression_level > 0 {
            stream.filter(Filter::FlateDecode);
        }
        stream.finish();
        Ok(pdf_writer.chunk)
    }
}

fn content_into_pdf(
    pdf_writer: &mut PdfWriter,
    content_map: &ContentMap,
    content_to_ref: &HashMap<ContentId, Ref>,
    rect: &Rectangle,
    content_id: ContentId,
) {
    let content = content_map.get(&content_id).unwrap();
    if let Some(rf) = content_to_ref.get(&content_id) {
        let (width, height) = content.size();
        pdf_writer.put_x_object(*rf, rect.clone(), width, height);
    } else {
        match content.body() {
            ContentBody::Text((text, is_shared)) => {
                assert!(!is_shared);
                let (width, height) = content.size();
                let rect = rect.fit_content_with_aspect_ratio(width, height);
                pdf_writer.content.save_state();
                pdf_writer.content.transform([
                    rect.width / width,
                    0.0,
                    0.0,
                    rect.height / height,
                    rect.x,
                    rect.y,
                ]);
                for path in text.paths() {
                    path_to_pdf(pdf_writer, path)
                }
                pdf_writer.content.restore_state();
            }
            ContentBody::BinImage(_) | ContentBody::SvgImage(_) => {
                unreachable!()
            }
            ContentBody::Composition(items) => {
                let (width, height) = content.size();
                let rect = rect.fit_content_with_aspect_ratio(width, height);
                pdf_writer.content.save_state();
                pdf_writer.content.transform([
                    rect.width / width,
                    0.0,
                    0.0,
                    rect.height / height,
                    rect.x,
                    rect.y,
                ]);
                for (r, c_id) in items {
                    content_into_pdf(pdf_writer, content_map, content_to_ref, r, *c_id);
                }
                pdf_writer.content.restore_state();
            }
        }
    }
}
