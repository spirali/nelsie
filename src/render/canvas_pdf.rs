use crate::common::{Color, DrawItem, DrawRect, Path, PathPart, Rectangle, Stroke};
use crate::model::Resources;
use crate::render::canvas::{Canvas, CanvasItem, Link};

use crate::common::error::NelsieError;
use crate::render::pdf::PdfRefAllocator;
use by_address::ByAddress;
use pdf_writer::types::{ActionType, AnnotationType};
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
        fn put_x_object(content: &mut Content, name: &str, rect: Rectangle, height: f32) {
            content
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
        }

        // First reference get from allocator is already preregistered as page reference
        let page_ref = alloc_ref.bump();

        let mut content = Content::new();
        content.save_state();
        let [r, g, b] = self.bg_color.as_f32s();
        content.set_fill_rgb(r, g, b);
        content.rect(0.0, 0.0, self.width, self.height);
        content.fill_nonzero();
        content.restore_state();

        let mut chunk = Chunk::new();
        let mut x_objects: Vec<(String, Ref)> = Vec::new();

        for (i, item) in self.items.into_iter().enumerate() {
            match item {
                CanvasItem::PngImage { rect, data } | CanvasItem::JpegImage { rect, data } => {
                    let (img_ref, _) = cache.get(&ByAddress(data)).unwrap();
                    let name = format!("o{i}");
                    put_x_object(&mut content, &name, rect, self.height);
                    x_objects.push((name, *img_ref));
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
                    let svg_ref = renumber_into(&svg_chunk, &mut chunk, alloc_ref, svg_ref);
                    let name = format!("o{i}");
                    put_x_object(&mut content, &name, rect, self.height);
                    x_objects.push((name, svg_ref));
                }
                CanvasItem::Text { text, x, y } => {
                    for path in text.paths() {
                        path_to_pdf_at(&mut content, path, x, y, self.height)
                    }
                }
                CanvasItem::DrawItems(items) => {
                    draw_items_to_pdf(&mut content, &items, self.height);
                }
            }
        }
        let annotation_ids = annotations_to_pdf(&mut chunk, alloc_ref, self.links, self.height);
        let content_ref = alloc_ref.bump();
        let mut page = chunk.page(page_ref);
        page.media_box(Rect::new(0.0, 0.0, self.width, self.height));
        page.parent(page_tree_ref);
        page.contents(content_ref);
        if !annotation_ids.is_empty() {
            page.annotations(annotation_ids);
        }

        let mut resources = page.resources();
        let mut objects = resources.x_objects();
        for (name, rf) in x_objects {
            objects.pair(Name(name.as_bytes()), rf);
        }
        objects.finish();
        resources.finish();

        page.finish();

        let mut content_data = content.finish();
        if compression_level > 0 {
            content_data =
                miniz_oxide::deflate::compress_to_vec_zlib(&content_data, compression_level)
        }
        let mut stream = chunk.stream(content_ref, &content_data);
        if compression_level > 0 {
            stream.filter(Filter::FlateDecode);
        }
        stream.finish();
        Ok(chunk)
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

fn draw_rect_to_pdf(content: &mut Content, item: &DrawRect) {
    set_fill_and_stroke_to_pdf(content, &item.fill_color, &item.stroke);
    content.rect(
        item.rectangle.x,
        item.rectangle.y,
        item.rectangle.width,
        item.rectangle.height,
    );
    draw_fill_and_stroke_to_pdf(content, &item.fill_color, &item.stroke);
}

fn draw_items_to_pdf(content: &mut Content, items: &[DrawItem], height: f32) {
    content.save_state();
    content.transform([1.0, 0.0, 0.0, -1.0, 0.0, height]);
    for item in items {
        content.save_state();
        match item {
            DrawItem::Rect(rect) => draw_rect_to_pdf(content, rect),
            DrawItem::Oval(_) => {
                todo!()
            }
            DrawItem::Path(path) => path_body_to_pdf(content, path),
        }
        content.restore_state();
    }
    content.restore_state();
}

fn annotations_to_pdf(
    chunk: &mut Chunk,
    alloc_ref: &mut PdfRefAllocator,
    links: Vec<Link>,
    height: f32,
) -> Vec<Ref> {
    let mut annotation_ids = Vec::with_capacity(links.len());
    for link in links {
        let rect = link.rect();
        let annotation_id = alloc_ref.bump();
        let mut annotation = chunk.annotation(annotation_id);
        annotation_ids.push(annotation_id);
        annotation.subtype(AnnotationType::Link);
        annotation.border(0.0, 0.0, 0.0, None);
        annotation.rect(Rect::new(
            rect.x,
            height - rect.y,
            rect.x + rect.width,
            height - (rect.y + rect.height),
        ));
        annotation
            .action()
            .action_type(ActionType::Uri)
            .uri(Str(link.url().as_bytes()));
        annotation.finish();
    }
    annotation_ids
}

fn set_fill_and_stroke_to_pdf(
    content: &mut Content,
    fill_color: &Option<Color>,
    stroke: &Option<Stroke>,
) {
    if let Some(color) = fill_color {
        let [r, g, b] = color.as_f32s();
        content.set_fill_rgb(r, g, b);
    }
    if let Some(stroke) = stroke {
        let [r, g, b] = stroke.color.as_f32s();
        content.set_stroke_rgb(r, g, b);
        content.set_line_width(stroke.width);
        if let Some(array) = &stroke.dash_array {
            content.set_dash_pattern(array.clone(), stroke.dash_offset);
        }
    }
}

fn draw_fill_and_stroke_to_pdf(
    content: &mut Content,
    fill_color: &Option<Color>,
    stroke: &Option<Stroke>,
) {
    match (fill_color.is_some(), stroke.is_some()) {
        (true, true) => content.fill_nonzero_and_stroke(),
        (true, false) => content.fill_nonzero(),
        (false, true) => content.stroke(),
        (false, false) => content.end_path(),
    };
}

fn path_body_to_pdf(content: &mut Content, path: &Path) {
    // Taken from resvg
    fn calc(n1: f32, n2: f32) -> f32 {
        (n1 + n2 * 2.0) / 3.0
    }

    set_fill_and_stroke_to_pdf(content, path.fill_color(), path.stroke());

    let mut last = (0.0, 0.0);
    for part in path.parts() {
        match part {
            PathPart::Move { x, y } => {
                last = (*x, *y);
                content.move_to(*x, *y);
            }
            PathPart::Line { x, y } => {
                last = (*x, *y);
                content.line_to(*x, *y);
            }
            PathPart::Quad { x1, y1, x, y } => {
                content.cubic_to(
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
                content.cubic_to(*x1, *y1, *x2, *y2, *x, *y);
            }
            PathPart::Close => {
                content.close_path();
            }
        }
    }
    draw_fill_and_stroke_to_pdf(content, path.fill_color(), path.stroke());
}

fn path_to_pdf_at(content: &mut Content, path: &Path, x: f32, y: f32, height: f32) {
    content.save_state();
    content.transform([1.0, 0.0, 0.0, -1.0, x, height - y]);
    path_body_to_pdf(content, path);
    content.restore_state();
}
