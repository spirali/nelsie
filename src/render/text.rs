use crate::common::{Color, Path, PathBuilder, Rectangle, Stroke};
use crate::model::{
    InTextAnchor, InTextBoxId, NodeId, PartialTextStyle, StyledText, TextAlign, TextStyle,
};
use fontique::Stretch;
use parley::fontique::Weight;
use parley::layout::{Alignment, GlyphRun, PositionedLayoutItem};
use parley::style::{FontStack, FontStyle, StyleProperty};
use parley::{fontique, FontContext, InlineBox, Layout, LayoutContext, RangedBuilder};
use resvg::usvg::FontStretch;
use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::raw::FontRef as ReadFontsRef;
use skrifa::{GlyphId, MetadataProvider};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub(crate) struct TextContext {
    pub layout_cx: LayoutContext<Color>,
    pub font_cx: FontContext,
}

#[derive(Debug)]
pub(crate) struct RenderedText {
    paths: Vec<Path>,
    width: f32,
    height: f32,

    line_layouts: Vec<Rectangle>,
    intext_rects: HashMap<InTextBoxId, Rectangle>,
}

impl RenderedText {
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    pub fn line_layouts(&self) -> &[Rectangle] {
        &self.line_layouts
    }

    pub fn intext_rects(&self) -> &HashMap<InTextBoxId, Rectangle> {
        &self.intext_rects
    }

    pub fn render(
        text_context: &mut TextContext,
        text: &StyledText,
        text_align: TextAlign,
    ) -> Self {
        let mut layout = styled_text_to_parley(text_context, text);

        layout.break_all_lines(None);
        layout.align(
            None,
            match text_align {
                TextAlign::Start => Alignment::Start,
                TextAlign::Center => Alignment::Middle,
                TextAlign::End => Alignment::End,
            },
        );

        let mut intext_rects = HashMap::new();
        let mut paths = Vec::new();
        let mut line_layouts = Vec::with_capacity(layout.len());
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
                        let id = (inline_box.id / 2) as u32;
                        if inline_box.id % 2 == 0 {
                            intext_rects.insert(
                                id,
                                Rectangle::new(inline_box.x, metrics.min_coord, 0.0, line_height),
                            );
                        } else {
                            let r = intext_rects.get_mut(&id).unwrap();
                            r.width = inline_box.x - r.x;
                        }
                    }
                };
            }
            if min_x.is_infinite() {
                min_x = 0.0;
                max_x = 0.0;
            }
            line_layouts.push(Rectangle::new(min_x, line_y, max_x - min_x, line_height));
        }
        RenderedText {
            paths,
            width: layout.width(),
            height: layout.height(),
            line_layouts,
            intext_rects,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct TextCache {
    cache: BTreeMap<NodeId, Arc<RenderedText>>,
}

impl TextCache {
    pub fn get_or_create(
        &mut self,
        node_id: NodeId,
        text_context: &mut TextContext,
        styled_text: &StyledText,
        text_align: TextAlign,
    ) -> &Arc<RenderedText> {
        // if let Some(rtext) = self.cache.get(&node_id) {
        //     return &rtext;
        // }
        // let rtext = RenderedText::render(text_context, styled_text);
        // self.cache.insert(node_id, rtext);
        // self.cache.get(&node_id).unwrap()
        self.cache.entry(node_id).or_insert_with(|| {
            Arc::new(RenderedText::render(text_context, styled_text, text_align))
        })
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Arc<RenderedText>> {
        self.cache.get(&node_id)
    }
}

fn set_text_style_to_parley(
    text_style: &PartialTextStyle,
    builder: &mut RangedBuilder<Color>,
    start: usize,
    end: usize,
) {
    let PartialTextStyle {
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
            StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(&font.family_name))),
            start..end,
        );
    }

    if let Some(color) = *color {
        builder.push(StyleProperty::Brush(color), start..end);
    }

    if let Some(size) = size {
        builder.push(StyleProperty::FontSize(*size), start..end);
    }

    if let Some(line_spacing) = line_spacing {
        builder.push_default(StyleProperty::LineHeight(*line_spacing));
    }

    if let Some(weight) = weight {
        builder.push(
            StyleProperty::FontWeight(Weight::new(*weight as f32)),
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
            StyleProperty::FontStretch(font_stretch_to_parley(*stretch)),
            start..end,
        );
    }
}

fn font_stretch_to_parley(stretch: FontStretch) -> Stretch {
    match stretch {
        FontStretch::UltraCondensed => Stretch::ULTRA_CONDENSED,
        FontStretch::ExtraCondensed => Stretch::EXTRA_CONDENSED,
        FontStretch::Condensed => Stretch::CONDENSED,
        FontStretch::SemiCondensed => Stretch::SEMI_CONDENSED,
        FontStretch::Normal => Stretch::NORMAL,
        FontStretch::SemiExpanded => Stretch::SEMI_EXPANDED,
        FontStretch::Expanded => Stretch::EXPANDED,
        FontStretch::ExtraExpanded => Stretch::EXTRA_EXPANDED,
        FontStretch::UltraExpanded => Stretch::ULTRA_EXPANDED,
    }
}

fn styled_text_to_parley(
    text_context: &mut TextContext,
    styled_text: &StyledText,
) -> Layout<Color> {
    let mut text = String::new();
    for line in &styled_text.styled_lines {
        text.push_str(&line.text);
        text.push('\n')
    }
    text.pop();
    let mut builder = text_context
        .layout_cx
        .ranged_builder(&mut text_context.font_cx, &text, 1.0);
    let mut offset: usize = 0;

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
        &font.family_name,
    ))));
    builder.push_default(StyleProperty::Brush(*color));
    builder.push_default(StyleProperty::FontSize(*size));
    builder.push_default(StyleProperty::LineHeight(*line_spacing));
    builder.push_default(StyleProperty::FontWeight(Weight::new(*weight as f32)));
    builder.push_default(StyleProperty::Underline(*underline));
    builder.push_default(StyleProperty::Strikethrough(*line_through));
    builder.push_default(StyleProperty::FontStretch(font_stretch_to_parley(*stretch)));
    if *italic {
        builder.push_default(StyleProperty::FontStyle(FontStyle::Italic));
    }
    let anchors = &styled_text.anchors;
    for (l_idx, line) in styled_text.styled_lines.iter().enumerate() {
        let mut o = offset;
        for (s_idx, span) in line.spans.iter().enumerate() {
            try_insert_ibox(&mut builder, o, l_idx, s_idx, anchors);
            if let Some(style_idx) = span.style_idx {
                let style = &styled_text.styles[style_idx as usize];
                set_text_style_to_parley(style, &mut builder, o, o + span.length as usize);
            }
            o += span.length as usize;
        }
        try_insert_ibox(&mut builder, o, l_idx, line.spans.len(), anchors);
        offset += line.text.len() + 1;
    }
    builder.build(&text)
}

fn try_insert_ibox(
    builder: &mut RangedBuilder<Color>,
    offset: usize,
    line_idx: usize,
    span_idx: usize,
    anchors: &HashMap<InTextBoxId, InTextAnchor>,
) {
    for (id, anchor) in anchors {
        if anchor.start.line_idx == line_idx as u32 && anchor.start.span_idx == span_idx as u32 {
            builder.push_inline_box(InlineBox {
                id: (*id as u64) * 2,
                index: offset,
                width: 0.0,
                height: 0.0,
            })
        }
        if anchor.end.line_idx == line_idx as u32 && anchor.end.span_idx == span_idx as u32 {
            builder.push_inline_box(InlineBox {
                id: (*id as u64) * 2 + 1,
                index: offset,
                width: 0.0,
                height: 0.0,
            })
        }
    }
}

fn render_decoration(glyph_run: &GlyphRun<Color>, color: Color, offset: f32, width: f32) -> Path {
    let y = glyph_run.baseline() - offset + width / 2.;
    let mut builder = PathBuilder::new(
        Some(Stroke {
            color,
            width,
            dash_array: None,
            dash_offset: 0.0,
        }),
        None,
    );
    builder.move_to(glyph_run.offset(), y);
    builder.line_to(glyph_run.offset() + glyph_run.advance(), y);
    builder.build()
}

fn render_glyph_run(glyph_run: &GlyphRun<Color>, out: &mut Vec<Path>) {
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
        path_builder: PathBuilder::new(None, Some(color)),
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
