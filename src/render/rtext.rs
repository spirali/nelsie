use crate::common::{Color, Path, PathBuilder};
use crate::model::{DrawingPath, NodeId, Resources, StyledText, TextStyle};
use image::{Pixel, Rgba, RgbaImage};
use parley::builder::RangedBuilder;
use parley::layout::{Alignment, Glyph, GlyphRun, PositionedLayoutItem};
use parley::style::{Brush, FontStack, StyleProperty};
use parley::swash::scale::ScaleContext;
use parley::{FontContext, Layout, LayoutContext};
use resvg::tiny_skia::{FillRule, Transform};
use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::raw::FontRef as ReadFontsRef;
use skrifa::{GlyphId, MetadataProvider};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;

pub(crate) struct TextContext {
    pub layout_cx: LayoutContext<Color>,
    pub font_cx: FontContext,
}

#[derive(Debug)]
pub(crate) struct RenderedText {
    paths: Vec<Path>,
    width: f32,
    height: f32,
}

impl RenderedText {
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    pub fn render(text_context: &mut TextContext, text: &StyledText) -> Self {
        let mut layout = styled_text_to_parley(text_context, text);

        layout.break_all_lines(None);
        layout.align(None, Alignment::Start);

        let mut paths = Vec::new();
        for line in layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        paths.push(render_glyph_run(&glyph_run));
                    }
                    PositionedLayoutItem::InlineBox(inline_box) => {
                        todo!()
                    }
                };
            }
        }
        RenderedText {
            paths,
            width: layout.width(),
            height: layout.height(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct TextCache {
    cache: BTreeMap<NodeId, RenderedText>,
}

impl TextCache {
    pub fn get_or_create(
        &mut self,
        node_id: NodeId,
        text_context: &mut TextContext,
        styled_text: &StyledText,
    ) -> &RenderedText {
        // if let Some(rtext) = self.cache.get(&node_id) {
        //     return &rtext;
        // }
        // let rtext = RenderedText::render(text_context, styled_text);
        // self.cache.insert(node_id, rtext);
        // self.cache.get(&node_id).unwrap()
        self.cache
            .entry(node_id)
            .or_insert_with(|| RenderedText::render(text_context, styled_text))
    }

    pub fn get(&self, node_id: NodeId) -> Option<&RenderedText> {
        self.cache.get(&node_id)
    }
}

fn set_text_style_to_parley(
    text_style: &TextStyle,
    builder: &mut RangedBuilder<Color>,
    start: usize,
    end: usize,
) {
    let TextStyle {
        font,
        stroke,
        color,
        size,
        line_spacing,
        italic,
        stretch,
        weight,
        underline,
        overline,
        line_through,
    } = text_style;
    let font_stack = FontStack::Source(&font.family_name);
    let font_stack_style: StyleProperty<Color> = StyleProperty::FontStack(font_stack);
    builder.push(&font_stack_style, start..end);

    if let Some(color) = *color {
        builder.push(&StyleProperty::Brush(color), start..end);
    }
    builder.push(&StyleProperty::FontSize(*size), start..end);

    //
    // let text_color = Color::from_str("black").unwrap();
    // builder.push(&brush_style, 0..text.len());
    // let font_stack_style: StyleProperty<Color> = StyleProperty::FontStack(font_stack);
    // builder.push(&font_stack_style, 0..text.len());
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
    for line in &styled_text.styled_lines {
        let mut o = offset;
        for span in &line.spans {
            let style = &styled_text.styles[span.style_idx as usize];
            set_text_style_to_parley(style, &mut builder, o, o + span.length as usize);
            o += span.length as usize;
        }
        offset += line.text.len() + 1;
    }
    builder.build(&text)
}

fn render_glyph_run(glyph_run: &GlyphRun<Color>) -> Path {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;

    // Get the "Run" from the "GlyphRun"
    let run = glyph_run.run();

    // Resolve properties of the Run
    let font = run.font();
    let font_size = run.font_size();

    let normalized_coords = run
        .normalized_coords()
        .iter()
        .map(|coord| NormalizedCoord::from_bits(*coord))
        .collect::<Vec<_>>();

    // Get glyph outlines using Skrifa. This can be cached in production code.
    let font_collection_ref = font.data.as_ref();
    let font_ref = ReadFontsRef::from_index(font_collection_ref, font.index).unwrap();
    let outlines = font_ref.outline_glyphs();

    let mut pen = NelsiePathPen {
        path_builder: PathBuilder::new(None, Some(color)),
        x: 0.0,
        y: 0.0,
    };
    let location_ref = LocationRef::new(&normalized_coords);

    // Iterates over the glyphs in the GlyphRun
    for glyph in glyph_run.glyphs() {
        pen.x = run_x + glyph.x;
        pen.y = run_y - glyph.y;
        run_x += glyph.advance;

        let glyph_id = GlyphId::from(glyph.id);
        let glyph_outline = outlines.get(glyph_id).unwrap();

        let settings = DrawSettings::unhinted(Size::new(font_size), location_ref);
        glyph_outline.draw(settings, &mut pen).unwrap();
    }
    pen.path_builder.build()
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
