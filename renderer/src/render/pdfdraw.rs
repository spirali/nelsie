use crate::render::canvas::Link;
use crate::render::composer_pdf::PdfRefAllocator;
use crate::render::draw::{DrawItem, DrawPath, DrawPathPart, DrawRect, PathBuilder};
use crate::shapes::FillAndStroke;
use crate::{Color, Rectangle};
use pdf_writer::types::{ActionType, AnnotationType};
use pdf_writer::{Chunk, Content, Finish, Name, Rect, Ref, Str};
use std::collections::HashMap;

pub struct PdfWriter<'a> {
    pub(crate) content: Content,
    pub(crate) chunk: Chunk,
    pub(crate) alloc_ref: &'a PdfRefAllocator,
    res_name_counter: u32,
    pub(crate) xo_resources: Vec<(String, Ref)>,
    pub(crate) gs_resources: HashMap<(u8, u8), (String, Ref)>,
}

impl<'a> PdfWriter<'a> {
    pub fn new(alloc_ref: &'a PdfRefAllocator) -> Self {
        PdfWriter {
            content: Content::new(),
            chunk: Chunk::new(),
            alloc_ref,
            res_name_counter: 0,
            xo_resources: Vec::new(),
            gs_resources: HashMap::new(),
        }
    }

    pub fn new_name(&mut self) -> String {
        let name = format!("o{}", self.res_name_counter);
        self.res_name_counter += 1;
        name
    }

    pub fn register_gs(&mut self, key: (u8, u8), name: String, rf: Ref) {
        self.gs_resources.insert(key, (name, rf));
    }

    pub fn put_x_object(&mut self, rf: Ref, rect: Rectangle, orig_w: f32, orig_h: f32) {
        let rect = rect.fit_content_with_aspect_ratio(orig_w, orig_h);
        let name = self.new_name();
        self.content
            .save_state()
            .transform([
                rect.width,
                0.0,
                0.0,
                -rect.height,
                rect.x,
                rect.y + rect.height,
            ])
            .x_object(Name(name.as_bytes()))
            .restore_state();
        self.xo_resources.push((name, rf));
    }
}

pub(crate) fn init_pdf(
    pdf: &mut pdf_writer::Pdf,
    alloc_ref: &mut Ref,
    n_pages: usize,
) -> (Ref, Vec<Ref>) {
    let catalog_ref = alloc_ref.bump();
    let page_tree_ref = alloc_ref.bump();
    pdf.catalog(catalog_ref).pages(page_tree_ref);
    let page_refs: Vec<Ref> = (0..n_pages).map(|_| alloc_ref.bump()).collect();
    pdf.pages(page_tree_ref)
        .kids(page_refs.iter().copied())
        .count(page_refs.len() as i32);
    (page_tree_ref, page_refs)
}

fn check_alpha(color: Color) -> Option<u8> {
    let alpha = color.alpha();
    if alpha < u8::MAX { Some(alpha) } else { None }
}

fn set_fill_and_stroke(pdf_writer: &mut PdfWriter, fill_and_stroke: &FillAndStroke) {
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
        if let Some((name, _)) = pdf_writer.gs_resources.get(&key) {
            pdf_writer.content.set_parameters(Name(name.as_bytes()));
        } else {
            let gs_ref = pdf_writer.alloc_ref.bump();
            let mut gs = pdf_writer.chunk.ext_graphics(gs_ref);
            if let Some(alpha) = fill_alpha {
                gs.non_stroking_alpha(alpha as f32 / u8::MAX as f32);
            }
            if let Some(alpha) = stroke_alpha {
                gs.stroking_alpha(alpha as f32 / u8::MAX as f32);
            }
            gs.finish();
            let name = pdf_writer.new_name();
            pdf_writer.content.set_parameters(Name(name.as_bytes()));
            pdf_writer.register_gs(key, name, gs_ref);
        }
    }
    if let Some(color) = &fill_and_stroke.fill_color {
        let [r, g, b] = color.as_f32s();
        pdf_writer.content.set_fill_rgb(r, g, b);
    }
    if let Some(stroke) = &fill_and_stroke.stroke {
        let [r, g, b] = stroke.color.as_f32s();
        pdf_writer.content.set_stroke_rgb(r, g, b);
        pdf_writer.content.set_line_width(stroke.width);
        if let Some(array) = &stroke.dash_array {
            pdf_writer
                .content
                .set_dash_pattern(array.clone(), stroke.dash_offset);
        }
    }
}

fn draw_rect_to_pdf(pdf_writer: &mut PdfWriter, item: &DrawRect) {
    set_fill_and_stroke(pdf_writer, &item.fill_and_stroke);
    pdf_writer.content.rect(
        item.rectangle.x,
        item.rectangle.y,
        item.rectangle.width,
        item.rectangle.height,
    );
    draw_fill_and_stroke(pdf_writer, &item.fill_and_stroke);
}

pub(crate) fn draw_item_to_pdf(pdf_writer: &mut PdfWriter, item: &DrawItem) {
    pdf_writer.content.save_state();
    match item {
        DrawItem::Rect(rect) => draw_rect_to_pdf(pdf_writer, rect),
        DrawItem::Oval(rect) => draw_ellipse_to_pdf(pdf_writer, rect),
        DrawItem::Path(path) => path_to_pdf(pdf_writer, path),
    }
    pdf_writer.content.restore_state();
}

pub fn pdf_rect(rect: &Rectangle) -> Rect {
    Rect::new(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
}

pub(crate) fn annotations_to_pdf(
    pdf_writer: &mut PdfWriter,
    links: Vec<Link>,
    height: f32,
    annotation_ids: &mut Vec<Ref>,
) {
    for link in links {
        let annotation_id = pdf_writer.alloc_ref.bump();
        let mut annotation = pdf_writer.chunk.annotation(annotation_id);
        annotation_ids.push(annotation_id);
        annotation.subtype(AnnotationType::Link);
        annotation.border(0.0, 0.0, 0.0, None);
        annotation.rect(pdf_rect(&link.rect().invert_y_axis(height)));
        annotation
            .action()
            .action_type(ActionType::Uri)
            .uri(Str(link.url().as_bytes()));
        annotation.finish();
    }
}

fn draw_fill_and_stroke(pdf_writer: &mut PdfWriter, fill_and_stroke: &FillAndStroke) {
    match (
        fill_and_stroke.fill_color.is_some(),
        fill_and_stroke.stroke.is_some(),
    ) {
        (true, true) => pdf_writer.content.fill_nonzero_and_stroke(),
        (true, false) => pdf_writer.content.fill_nonzero(),
        (false, true) => pdf_writer.content.stroke(),
        (false, false) => pdf_writer.content.end_path(),
    };
}

fn draw_ellipse_to_pdf(pdf_writer: &mut PdfWriter, item: &DrawRect) {
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
    path_to_pdf(pdf_writer, &builder.build())
}

/*fn draw_video_to_pdf(
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
}*/

pub fn path_to_pdf(pdf_writer: &mut PdfWriter, path: &DrawPath) {
    // Taken from resvg
    fn calc(n1: f32, n2: f32) -> f32 {
        (n1 + n2 * 2.0) / 3.0
    }

    set_fill_and_stroke(pdf_writer, path.fill_and_stroke());

    let mut last = (0.0, 0.0);
    for part in path.parts() {
        match part {
            DrawPathPart::Move { x, y } => {
                last = (*x, *y);
                pdf_writer.content.move_to(*x, *y);
            }
            DrawPathPart::Line { x, y } => {
                last = (*x, *y);
                pdf_writer.content.line_to(*x, *y);
            }
            DrawPathPart::Quad { x1, y1, x, y } => {
                pdf_writer.content.cubic_to(
                    calc(last.0, *x1),
                    calc(last.1, *y1),
                    calc(*x, *x1),
                    calc(*y, *y1),
                    *x,
                    *y,
                );
                last = (*x, *y);
            }
            DrawPathPart::Cubic {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                last = (*x, *y);
                pdf_writer.content.cubic_to(*x1, *y1, *x2, *y2, *x, *y);
            }
            DrawPathPart::Close => {
                pdf_writer.content.close_path();
            }
        }
    }
    draw_fill_and_stroke(pdf_writer, path.fill_and_stroke());
}
