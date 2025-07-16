use crate::common::{
    Color, DrawItem, DrawRect, FillAndStroke, Path, PathBuilder, PathPart, Rectangle,
};
use crate::model::{LoadedImageData, Resources, Video};
use crate::render::canvas::{Canvas, CanvasItem, Link};

use crate::common::error::NelsieError;
use crate::render::pdf::PdfRefAllocator;
use by_address::ByAddress;
use pdf_writer::types::{
    ActionType, AnnotationType, MediaClipType, RenditionOperation, RenditionType, TempFileType,
};
use pdf_writer::{Chunk, Content, Filter, Finish, Name, Rect, Ref, Str};
use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;
use svg2pdf::usvg;

pub(crate) type PdfImageCache = HashMap<ByAddress<Arc<Vec<u8>>>, (Ref, Ref)>;

impl Canvas {
    pub fn into_pdf_page(
        self,
        resources: &Resources,
        alloc_ref: &mut PdfRefAllocator,
        page_tree_ref: Ref,
        cache: &PdfImageCache,
        compression_level: u8,
    ) -> crate::Result<Chunk> {
        // First reference get from allocator is already preregistered as page reference
        let page_ref = alloc_ref.bump();

        let mut content = Content::new();
        content.save_state();
        let [r, g, b] = self.bg_color.as_f32s();
        content.set_fill_rgb(r, g, b);
        content.rect(0.0, 0.0, self.width, self.height);
        content.fill_nonzero();
        content.restore_state();

        let mut pdf_ctx = PdfCtx {
            content,
            chunk: Chunk::new(),
            alloc_ref,
            res_name_counter: 0,
            xo_resources: Vec::new(),
            gs_resources: HashMap::new(),
        };

        let mut annotation_ids = Vec::new();

        for item in self.items.into_iter() {
            match item {
                CanvasItem::PngImage { rect, data } | CanvasItem::JpegImage { rect, data } => {
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
                            |e| NelsieError::generic_err(format!("PDF conversion error: {e}")),
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
                CanvasItem::DrawItems(items) => {
                    draw_items_to_pdf(&mut pdf_ctx, &items, self.height);
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
            }
        }
        annotations_to_pdf(&mut pdf_ctx, self.links, self.height, &mut annotation_ids);
        let content_ref = pdf_ctx.alloc_ref.bump();
        let mut page = pdf_ctx.chunk.page(page_ref);
        page.media_box(Rect::new(0.0, 0.0, self.width, self.height));
        page.parent(page_tree_ref);
        page.contents(content_ref);
        if !annotation_ids.is_empty() {
            page.annotations(annotation_ids);
        }

        let mut resources = page.resources();
        let mut objects = resources.x_objects();
        for (name, rf) in pdf_ctx.xo_resources {
            objects.pair(Name(name.as_bytes()), rf);
        }
        objects.finish();
        let mut g_states = resources.ext_g_states();
        for (name, rf) in pdf_ctx.gs_resources.values() {
            g_states.pair(Name(name.as_bytes()), rf);
        }
        g_states.finish();
        resources.finish();

        page.finish();

        let mut content_data = pdf_ctx.content.finish();
        if compression_level > 0 {
            content_data =
                miniz_oxide::deflate::compress_to_vec_zlib(&content_data, compression_level)
        }
        let mut stream = pdf_ctx.chunk.stream(content_ref, &content_data);
        if compression_level > 0 {
            stream.filter(Filter::FlateDecode);
        }
        stream.finish();
        Ok(pdf_ctx.chunk)
    }
}

struct PdfCtx<'a> {
    content: Content,
    chunk: Chunk,
    alloc_ref: &'a mut PdfRefAllocator,
    res_name_counter: u32,
    xo_resources: Vec<(String, Ref)>,
    gs_resources: HashMap<(u8, u8), (String, Ref)>,
}

impl PdfCtx<'_> {
    pub fn new_name(&mut self) -> String {
        let name = format!("o{}", self.res_name_counter);
        self.res_name_counter += 1;
        name
    }

    pub fn register_gs(&mut self, key: (u8, u8), name: String, rf: Ref) {
        self.gs_resources.insert(key, (name, rf));
    }

    fn put_x_object(&mut self, rf: Ref, rect: Rectangle, height: f32) {
        let name = self.new_name();
        self.content
            .save_state()
            .transform([
                rect.width,
                0.0,
                0.0,
                rect.height,
                rect.x,
                height - rect.height - rect.y,
            ])
            .x_object(Name(name.as_bytes()))
            .restore_state();
        self.xo_resources.push((name, rf));
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

fn draw_rect_to_pdf(pdf_ctx: &mut PdfCtx, item: &DrawRect) {
    set_fill_and_stroke(pdf_ctx, &item.fill_and_stroke);
    pdf_ctx.content.rect(
        item.rectangle.x,
        item.rectangle.y,
        item.rectangle.width,
        item.rectangle.height,
    );
    draw_fill_and_stroke(pdf_ctx, &item.fill_and_stroke);
}

fn draw_ellipse_to_pdf(pdf_ctx: &mut PdfCtx, item: &DrawRect) {
    let mut builder = PathBuilder::new(item.fill_and_stroke.clone());
    let rx = item.rectangle.width / 2.0;
    let ry = item.rectangle.height / 2.0;
    let cx = item.rectangle.x + rx;
    let cy = item.rectangle.y + ry;
    builder.move_to(cx + rx, cy);
    builder.arc_to(rx, ry, 0.0, false, true, cx, cy + ry);
    builder.arc_to(rx, ry, 0.0, false, true, cx - rx, cy);
    builder.arc_to(rx, ry, 0.0, false, true, cx, cy - ry);
    builder.arc_to(rx, ry, 0.0, false, true, cx + rx, cy);
    builder.close();
    path_body_to_pdf(pdf_ctx, &builder.build())
}

fn draw_video_to_pdf(
    pdf_ctx: &mut PdfCtx,
    rect: &Rectangle,
    video: &Arc<Video>,
    height: f32,
    page_ref: Ref,
    cache: &PdfImageCache,
    annotations: &mut Vec<Ref>,
) -> crate::Result<()> {
    let cover_form_ref = video.cover_image.as_ref().map(|image| {
        let image_data = match &image.data {
            LoadedImageData::Png(data) | LoadedImageData::Jpeg(data) => data,
            _ => unreachable!(),
        };
        let (cover_image_id, _) = cache.get(&ByAddress(image_data.clone())).unwrap();
        let form_xobject_ref = pdf_ctx.alloc_ref.bump();
        let image_name = format!("cover{}", form_xobject_ref.get());
        let mut content = Content::new();
        content.save_state();
        content.transform([
            rect.width,
            0.0,
            0.0,
            rect.height,
            rect.x,
            height - rect.height - rect.y,
        ]);
        content.x_object(Name(image_name.as_bytes()));
        content.restore_state();
        let content_data = content.finish();
        let mut form_xobject = pdf_ctx.chunk.form_xobject(form_xobject_ref, &content_data);
        form_xobject.bbox(pdf_rect(rect, height));
        form_xobject
            .resources()
            .x_objects()
            .pair(Name(image_name.as_bytes()), cover_image_id);
        form_xobject.finish();
        form_xobject_ref
    });
    let annotation_id = pdf_ctx.alloc_ref.bump();
    annotations.push(annotation_id);
    let video_file_id = pdf_ctx.alloc_ref.bump();
    {
        let data = std::fs::read(&video.path)?;
        pdf_ctx.chunk.embedded_file(video_file_id, &data);
    }
    let mut annotation = pdf_ctx.chunk.annotation(annotation_id);
    annotation.subtype(AnnotationType::Screen);
    annotation.rect(pdf_rect(rect, height));
    annotation.page(page_ref);
    if let Some(form_ref) = cover_form_ref {
        annotation.appearance().normal().stream(form_ref);
    }

    let mut action = annotation.action();
    action.action_type(ActionType::Rendition);
    action.operation(RenditionOperation::Play);
    action.annotation(annotation_id);

    let mut rendition = action.rendition();
    rendition.subtype(RenditionType::Media);

    let mut media_clip = rendition.media_clip();
    media_clip.subtype(MediaClipType::Data);
    media_clip.data().embedded_file(video_file_id);
    media_clip.data_type(Str(video.data_type.as_bytes()));
    media_clip.permissions().temp_file(TempFileType::Access);
    media_clip.finish();
    rendition.media_play_params().controls(video.show_controls);
    rendition.finish();
    action.finish();
    annotation.finish();
    Ok(())
}

fn draw_items_to_pdf(pdf_ctx: &mut PdfCtx, items: &[DrawItem], height: f32) {
    pdf_ctx.content.save_state();
    pdf_ctx
        .content
        .transform([1.0, 0.0, 0.0, -1.0, 0.0, height]);
    for item in items {
        pdf_ctx.content.save_state();
        match item {
            DrawItem::Rect(rect) => draw_rect_to_pdf(pdf_ctx, rect),
            DrawItem::Oval(rect) => draw_ellipse_to_pdf(pdf_ctx, rect),
            DrawItem::Path(path) => path_body_to_pdf(pdf_ctx, path),
        }
        pdf_ctx.content.restore_state();
    }
    pdf_ctx.content.restore_state();
}

fn pdf_rect(rect: &Rectangle, height: f32) -> Rect {
    Rect::new(
        rect.x,
        height - rect.y,
        rect.x + rect.width,
        height - (rect.y + rect.height),
    )
}

fn annotations_to_pdf(
    pdf_ctx: &mut PdfCtx,
    links: Vec<Link>,
    height: f32,
    annotation_ids: &mut Vec<Ref>,
) {
    for link in links {
        let rect = link.rect();
        let annotation_id = pdf_ctx.alloc_ref.bump();
        let mut annotation = pdf_ctx.chunk.annotation(annotation_id);
        annotation_ids.push(annotation_id);
        annotation.subtype(AnnotationType::Link);
        annotation.border(0.0, 0.0, 0.0, None);
        annotation.rect(pdf_rect(rect, height));
        annotation
            .action()
            .action_type(ActionType::Uri)
            .uri(Str(link.url().as_bytes()));
        annotation.finish();
    }
}

fn check_alpha(color: Color) -> Option<u8> {
    let alpha = color.alpha();
    if alpha < u8::MAX {
        Some(alpha)
    } else {
        None
    }
}

fn set_fill_and_stroke(pdf_ctx: &mut PdfCtx, fill_and_stroke: &FillAndStroke) {
    let fill_alpha = fill_and_stroke.fill_color.and_then(check_alpha);
    let stroke_alpha = fill_and_stroke
        .stroke
        .as_ref()
        .and_then(|stroke| check_alpha(stroke.color));
    if fill_alpha.is_some() || stroke_alpha.is_some() {
        let key = (
            fill_alpha.unwrap_or(u8::MAX),
            stroke_alpha.unwrap_or(u8::MAX),
        );
        if let Some((name, _)) = pdf_ctx.gs_resources.get(&key) {
            pdf_ctx.content.set_parameters(Name(name.as_bytes()));
        } else {
            let gs_ref = pdf_ctx.alloc_ref.bump();
            let mut gs = pdf_ctx.chunk.ext_graphics(gs_ref);
            if let Some(alpha) = fill_alpha {
                gs.non_stroking_alpha(alpha as f32 / u8::MAX as f32);
            }
            if let Some(alpha) = stroke_alpha {
                gs.stroking_alpha(alpha as f32 / u8::MAX as f32);
            }
            gs.finish();
            let name = pdf_ctx.new_name();
            pdf_ctx.content.set_parameters(Name(name.as_bytes()));
            pdf_ctx.register_gs(key, name, gs_ref);
        }
    }

    if let Some(color) = &fill_and_stroke.fill_color {
        let [r, g, b] = color.as_f32s();
        pdf_ctx.content.set_fill_rgb(r, g, b);
    }
    if let Some(stroke) = &fill_and_stroke.stroke {
        let [r, g, b] = stroke.color.as_f32s();
        pdf_ctx.content.set_stroke_rgb(r, g, b);
        pdf_ctx.content.set_line_width(stroke.width);
        if let Some(array) = &stroke.dash_array {
            pdf_ctx
                .content
                .set_dash_pattern(array.clone(), stroke.dash_offset);
        }
    }
}

fn draw_fill_and_stroke(pdf_ctx: &mut PdfCtx, fill_and_stroke: &FillAndStroke) {
    match (
        fill_and_stroke.fill_color.is_some(),
        fill_and_stroke.stroke.is_some(),
    ) {
        (true, true) => pdf_ctx.content.fill_nonzero_and_stroke(),
        (true, false) => pdf_ctx.content.fill_nonzero(),
        (false, true) => pdf_ctx.content.stroke(),
        (false, false) => pdf_ctx.content.end_path(),
    };
}

fn path_body_to_pdf(pdf_ctx: &mut PdfCtx, path: &Path) {
    // Taken from resvg
    fn calc(n1: f32, n2: f32) -> f32 {
        (n1 + n2 * 2.0) / 3.0
    }

    set_fill_and_stroke(pdf_ctx, path.fill_and_stroke());

    let mut last = (0.0, 0.0);
    for part in path.parts() {
        match part {
            PathPart::Move { x, y } => {
                last = (*x, *y);
                pdf_ctx.content.move_to(*x, *y);
            }
            PathPart::Line { x, y } => {
                last = (*x, *y);
                pdf_ctx.content.line_to(*x, *y);
            }
            PathPart::Quad { x1, y1, x, y } => {
                pdf_ctx.content.cubic_to(
                    calc(last.0, *x1),
                    calc(last.1, *y1),
                    calc(*x, *x1),
                    calc(*y, *y1),
                    *x,
                    *y,
                );
                last = (*x, *y);
            }
            PathPart::Cubic {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                last = (*x, *y);
                pdf_ctx.content.cubic_to(*x1, *y1, *x2, *y2, *x, *y);
            }
            PathPart::Close => {
                pdf_ctx.content.close_path();
            }
        }
    }
    draw_fill_and_stroke(pdf_ctx, path.fill_and_stroke());
}

fn path_to_pdf_at(pdf_ctx: &mut PdfCtx, path: &Path, x: f32, y: f32, height: f32) {
    pdf_ctx.content.save_state();
    pdf_ctx
        .content
        .transform([1.0, 0.0, 0.0, -1.0, x, height - y]);
    path_body_to_pdf(pdf_ctx, path);
    pdf_ctx.content.restore_state();
}
