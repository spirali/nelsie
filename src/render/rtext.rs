use crate::common::{Color, Path, PathBuilder};
use crate::model::{DrawingPath, StyledText};
use image::{Pixel, Rgba, RgbaImage};
use parley::layout::{Alignment, Glyph, GlyphRun, PositionedLayoutItem};
use parley::style::{Brush, FontStack, StyleProperty};
use parley::swash::scale::ScaleContext;
use parley::{FontContext, LayoutContext};
use resvg::tiny_skia::{FillRule, Transform};
use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::raw::FontRef as ReadFontsRef;
use skrifa::{GlyphId, MetadataProvider};
use std::str::FromStr;

pub(crate) struct RenderedText {
    paths: Vec<Path>,
}

impl RenderedText {
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    pub fn render(text: &StyledText) -> Self {
        // TODO Create just once
        let mut font_cx = FontContext::default();
        let mut layout_cx = LayoutContext::new();
        let mut scale_cx = ScaleContext::new();

        let font_stack = FontStack::Source("system-ui");

        let text = "Testing text\nLine2";
        let mut builder = layout_cx.ranged_builder(&mut font_cx, &text, 1.0);

        let text_color = Color::from_str("black").unwrap();
        let brush_style = StyleProperty::Brush(text_color);
        builder.push(&brush_style, 0..text.len());
        let font_stack_style: StyleProperty<Color> = StyleProperty::FontStack(font_stack);
        builder.push(&font_stack_style, 0..text.len());
        let mut layout = builder.build(&text);

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
        RenderedText { paths }
    }
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
