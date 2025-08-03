use crate::render::canvas::{Canvas, CanvasItem, Link};

use crate::render::composer_pdf::PdfRefAllocator;
use crate::render::content::{ContentBody, ContentMap};
use crate::render::draw::{DrawItem, DrawPath, DrawPathPart, DrawRect, PathBuilder};
use crate::render::pdfdraw::{PdfWriter, annotations_to_pdf, draw_item_to_pdf, path_to_pdf};
use crate::shapes::FillAndStroke;
use crate::{Color, ContentId, Rectangle};
use pdf_writer::types::{
    ActionType, AnnotationType, MediaClipType, RenditionOperation, RenditionType, TempFileType,
};
use pdf_writer::{Chunk, Content, Filter, Finish, Name, Rect, Ref, Str};
use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;
use svg2pdf::usvg;

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
            /*                CanvasItem::PngImage { rect, data } | CanvasItem::JpegImage { rect, data } => {
                    let (img_ref, _) = cache.get(&ByAddress(data)).unwrap();
                    pdf_ctx.put_x_object(*img_ref, rect, self.height);
                }
                CanvasItem::SvgImage {
                    rect,
                    data,
                    width: _,
                    height: _,
                } => {
                    let options = usvg::Options {
                        fontdb: resources.font_db.as_ref().unwrap().clone(),
                        ..Default::default()
                    };
                    let tree = usvg::Tree::from_str(&data, &options)?;
                    let (svg_chunk, svg_ref) =
                        svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default()).map_err(
                            |e| NelsieError::generic_err(format!("PDF conversion error: {}", e)),
                        )?;
                    let svg_ref =
                        renumber_into(&svg_chunk, &mut pdf_ctx.chunk, pdf_ctx.alloc_ref, svg_ref);
                    pdf_ctx.put_x_object(svg_ref, rect, self.height);
                }
                CanvasItem::Text { text, x, y } => {
                    for path in text.paths() {
                        path_to_pdf_at(&mut pdf_ctx, path, x, y, self.height)
                    }
                }
                CanvasItem::Video { rect, video } => {
                    draw_video_to_pdf(
                        &mut pdf_ctx,
                        &rect,
                        &video,
                        self.height,
                        page_ref,
                        cache,
                        &mut annotation_ids,
                    )?;
                }
            }*/
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
    if let Some(rf) = content_to_ref.get(&content_id) {
        pdf_writer.put_x_object(*rf, rect.clone());
    } else {
        let content = content_map.get(&content_id).unwrap();
        match content.body() {
            ContentBody::Text((text, is_shared)) => {
                assert!(!is_shared);
                let (width, height) = content.size();
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

fn renumber_into(
    chunk: &Chunk,
    target: &mut Chunk,
    alloc_ref: &mut PdfRefAllocator,
    top_ref: Ref,
) -> Ref {
    let mut map = HashMap::<Ref, Ref>::new();
    chunk.renumber_into(target, |r| {
        *map.entry(r).or_insert_with(|| alloc_ref.bump())
    });
    *map.get(&top_ref).unwrap()
}
