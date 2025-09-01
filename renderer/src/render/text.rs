use crate::render::draw::{DrawPath, PathBuilder};
use crate::shapes::{FillAndStroke, Stroke};
use crate::text::{InlineId, Text, TextAlign, TextStyle};
use crate::textutils::StyledText;
use crate::{Color, Rectangle, Resources};
use parley::{
    Alignment, AlignmentOptions, FontContext, FontStack, FontStyle, FontWeight, FontWidth,
    GlyphRun, InlineBox, Layout, LayoutContext, PositionedLayoutItem, RangedBuilder, StyleProperty,
};
use resvg::usvg::FontStretch;
use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::raw::FontRef as ReadFontsRef;
use skrifa::{GlyphId, MetadataProvider};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct RenderedText {
    paths: Vec<DrawPath>,
    line_rects: Vec<Rectangle>,
    inline_rects: HashMap<InlineId, Rectangle>,
}

impl RenderedText {
    pub fn paths(&self) -> &[DrawPath] {
        &self.paths
    }

    pub fn line_layouts(&self) -> &[Rectangle] {
        &self.line_rects
    }

    pub fn intext_rects(&self) -> &HashMap<InlineId, Rectangle> {
        &self.inline_rects
    }
}

pub(crate) struct TextContext {
    pub layout_cx: LayoutContext<Color>,
    pub font_cx: FontContext,
}

pub fn render_text(
    resources: &Resources,
    text_ctx: &mut TextContext,
    text: &Text,
) -> crate::Result<(RenderedText, f32, f32)> {
    let styled_text = StyledText::from(resources, text)?;

    let mut layout = styled_text_to_parley(text_ctx, &styled_text);

    layout.break_all_lines(None);
    layout.align(
        None,
        match text.text_align {
            TextAlign::Start => Alignment::Start,
            TextAlign::Center => Alignment::Middle,
            TextAlign::End => Alignment::End,
        },
        AlignmentOptions::default(),
    );

    let mut inline_rects = HashMap::new();
    let mut paths = Vec::new();
    let mut line_rects = Vec::with_capacity(layout.len());
    for line in layout.lines() {
        let mut min_x: f32 = f32::INFINITY;
        let mut max_x: f32 = 0.0;
        let metrics = line.metrics();
        let line_y = metrics.min_coord;
        let line_height = metrics.max_coord - metrics.min_coord;
        for item in line.items() {
            match item {
                PositionedLayoutItem::GlyphRun(glyph_run) => {
                    render_glyph_run(&glyph_run, &mut paths);
                    min_x = min_x.min(glyph_run.offset());
                    max_x = max_x.max(glyph_run.offset() + glyph_run.advance());
                }
                PositionedLayoutItem::InlineBox(inline_box) => {
                    let id = InlineId::new((inline_box.id / 2) as u32);
                    if inline_box.id % 2 == 0 {
                        inline_rects.insert(
                            id,
                            Rectangle::new(inline_box.x, metrics.min_coord, 0.0, line_height),
                        );
                    } else {
                        let r = inline_rects.get_mut(&id).unwrap();
                        r.width = inline_box.x - r.x;
                    }
                }
            };
        }
        if min_x.is_infinite() {
            min_x = 0.0;
            max_x = 0.0;
        }
        line_rects.push(Rectangle::new(min_x, line_y, max_x - min_x, line_height));
    }

    Ok((
        RenderedText {
            paths,
            line_rects,
            inline_rects,
        },
        layout.width(),
        layout.height(),
    ))
}

fn styled_text_to_parley(
    text_context: &mut TextContext,
    styled_text: &StyledText,
) -> Layout<Color> {
    let mut builder =
        text_context
            .layout_cx
            .ranged_builder(&mut text_context.font_cx, &styled_text.text, 1.0);
    let TextStyle {
        font,
        color,
        size,
        line_spacing,
        italic,
        stretch,
        weight,
        underline,
        line_through,
    } = &styled_text.main_style;
    builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(
        font.as_ref().map(|x| x.as_str()).unwrap_or("sans-serif"),
    ))));
    builder.push_default(StyleProperty::Brush((*color).unwrap_or_default()));
    builder.push_default(StyleProperty::FontSize(
        (*size).map(|x| x.get()).unwrap_or(16.0),
    ));
    builder.push_default(StyleProperty::LineHeight(
        (*line_spacing).map(|x| x.get()).unwrap_or(1.0),
    ));
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(
        (*weight).unwrap_or(400) as f32,
    )));
    builder.push_default(StyleProperty::Underline((*underline).unwrap_or_default()));
    builder.push_default(StyleProperty::Strikethrough(
        (*line_through).unwrap_or_default(),
    ));
    builder.push_default(StyleProperty::FontWidth(
        (*stretch)
            .map(font_stretch_to_parley)
            .unwrap_or(FontWidth::NORMAL),
    ));
    if *italic == Some(true) {
        builder.push_default(StyleProperty::FontStyle(FontStyle::Italic));
    }
    for style in &styled_text.styles {
        set_text_style_to_parley(
            &mut builder,
            &style.style,
            style.start as usize,
            style.end as usize,
        );
    }
    for (anchor_id, anchor) in &styled_text.anchors {
        let id = anchor_id.as_u32() as u64 * 2;
        builder.push_inline_box(InlineBox {
            id,
            index: anchor.start as usize,
            width: 0.0,
            height: 0.0,
        });
        builder.push_inline_box(InlineBox {
            id: id + 1,
            index: anchor.end as usize,
            width: 0.0,
            height: 0.0,
        });
    }
    builder.build(&styled_text.text)
}

fn render_decoration(
    glyph_run: &GlyphRun<Color>,
    color: Color,
    offset: f32,
    width: f32,
) -> DrawPath {
    let y = glyph_run.baseline() - offset + width / 2.;
    let mut builder = PathBuilder::new(FillAndStroke::new_stroke(Stroke {
        color,
        width,
        dash_array: None,
        dash_offset: 0.0,
    }));
    builder.move_to(glyph_run.offset(), y);
    builder.line_to(glyph_run.offset() + glyph_run.advance(), y);
    builder.build()
}

fn render_glyph_run(glyph_run: &GlyphRun<Color>, out: &mut Vec<DrawPath>) {
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;
    let run = glyph_run.run();
    let font = run.font();
    let font_size = run.font_size();
    let normalized_coords = run
        .normalized_coords()
        .iter()
        .map(|coord| NormalizedCoord::from_bits(*coord))
        .collect::<Vec<_>>();

    let font_collection_ref = font.data.as_ref();
    let font_ref = ReadFontsRef::from_index(font_collection_ref, font.index).unwrap();
    let outlines = font_ref.outline_glyphs();

    let mut pen = NelsiePathPen {
        path_builder: PathBuilder::new(FillAndStroke::new_fill(color)),
        x: 0.0,
        y: 0.0,
    };
    let location_ref = LocationRef::new(&normalized_coords);
    for glyph in glyph_run.glyphs() {
        pen.x = run_x + glyph.x;
        pen.y = run_y - glyph.y;
        run_x += glyph.advance;

        let glyph_id = GlyphId::from(glyph.id);
        let glyph_outline = outlines.get(glyph_id).unwrap();

        let settings = DrawSettings::unhinted(Size::new(font_size), location_ref);
        glyph_outline.draw(settings, &mut pen).unwrap();
    }
    out.push(pen.path_builder.build());

    let style = glyph_run.style();
    let run_metrics = run.metrics();
    if let Some(decoration) = &style.underline {
        let offset = decoration.offset.unwrap_or(run_metrics.underline_offset);
        let size = decoration.size.unwrap_or(run_metrics.underline_size);
        out.push(render_decoration(glyph_run, decoration.brush, offset, size));
    }
    if let Some(decoration) = &style.strikethrough {
        let offset = decoration
            .offset
            .unwrap_or(run_metrics.strikethrough_offset);
        let size = decoration.size.unwrap_or(run_metrics.strikethrough_size);
        out.push(render_decoration(glyph_run, decoration.brush, offset, size));
    }
}

struct NelsiePathPen {
    path_builder: PathBuilder,
    x: f32,
    y: f32,
}

impl OutlinePen for NelsiePathPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_builder.move_to(self.x + x, self.y - y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path_builder.line_to(self.x + x, self.y - y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.path_builder
            .quad_to(self.x + cx0, self.y - cy0, self.x + x, self.y - y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.path_builder.cubic_to(
            self.x + cx0,
            self.y - cy0,
            self.x + cx1,
            self.y - cy1,
            self.x + x,
            self.y - y,
        );
    }

    fn close(&mut self) {
        self.path_builder.close();
    }
}

fn font_stretch_to_parley(stretch: FontStretch) -> FontWidth {
    match stretch {
        FontStretch::UltraCondensed => FontWidth::ULTRA_CONDENSED,
        FontStretch::ExtraCondensed => FontWidth::EXTRA_CONDENSED,
        FontStretch::Condensed => FontWidth::CONDENSED,
        FontStretch::SemiCondensed => FontWidth::SEMI_CONDENSED,
        FontStretch::Normal => FontWidth::NORMAL,
        FontStretch::SemiExpanded => FontWidth::SEMI_EXPANDED,
        FontStretch::Expanded => FontWidth::EXPANDED,
        FontStretch::ExtraExpanded => FontWidth::EXTRA_EXPANDED,
        FontStretch::UltraExpanded => FontWidth::ULTRA_EXPANDED,
    }
}

fn set_text_style_to_parley(
    builder: &mut RangedBuilder<Color>,
    text_style: &TextStyle,
    start: usize,
    end: usize,
) {
    let TextStyle {
        font,
        color,
        size,
        line_spacing,
        italic,
        stretch,
        weight,
        underline,
        line_through,
    } = text_style;

    if let Some(font) = font {
        builder.push(
            StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(font.as_str()))),
            start..end,
        );
    }

    if let Some(color) = *color {
        builder.push(StyleProperty::Brush(color), start..end);
    }

    if let Some(size) = size {
        builder.push(StyleProperty::FontSize(size.get()), start..end);
    }

    if let Some(line_spacing) = line_spacing {
        builder.push_default(StyleProperty::LineHeight(line_spacing.get()));
    }

    if let Some(weight) = weight {
        builder.push(
            StyleProperty::FontWeight(FontWeight::new(*weight as f32)),
            start..end,
        );
    }

    if let Some(line_through) = line_through {
        builder.push(StyleProperty::Strikethrough(*line_through), start..end);
    }

    if let Some(underline) = underline {
        builder.push(StyleProperty::Underline(*underline), start..end);
    }

    if let Some(italic) = italic {
        builder.push(
            StyleProperty::FontStyle(if *italic {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            }),
            start..end,
        );
    };
    if let Some(stretch) = stretch {
        builder.push(
            StyleProperty::FontWidth(font_stretch_to_parley(*stretch)),
            start..end,
        );
    }
}
