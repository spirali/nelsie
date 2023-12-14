use crate::model::{Span, StyledLine, StyledText, TextStyle};

use crate::render::paths::stroke_to_usvg_stroke;
use usvg::{fontdb, NonZeroPositiveF32, TreeTextToPath};
use usvg_tree::{
    AlignmentBaseline, CharacterPosition, DominantBaseline, Fill, Font, FontStyle,
    LengthAdjust, NodeKind, PaintOrder, Text, TextAnchor, TextChunk, TextDecoration, TextFlow,
    TextRendering, TextSpan, Visibility, WritingMode,
};

pub(crate) fn get_text_size(font_db: &fontdb::Database, text: &StyledText) -> (f32, f32) {
    let text_node = render_text(text, 0.0, 0.0);
    let root_node = usvg::Node::new(NodeKind::Group(usvg::Group::default()));
    root_node.append(text_node);
    let size = usvg::Size::from_wh(800.0, 600.0).unwrap();
    let mut tree = usvg_tree::Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_node,
    };
    tree.convert_text(font_db);
    let mut width: f32 = 0.0;
    if let Some(child) = tree.root.first_child() {
        let mut children = child.children();
        for line in text.styled_lines {
            width = width.max(
                (0..line.spans.len())
                    .map(|_| {
                        let child = children.next().unwrap();
                        let borrowed = child.borrow();
                        match *borrowed {
                            NodeKind::Path(ref path) => {
                                let bbox = path.text_bbox.unwrap();
                                bbox.width()
                            }
                            _ => unreachable!(),
                        }
                    })
                    .sum(),
            );
        }
    }
    (width, text.height())
}

fn create_svg_span(text_styles: &[TextStyle], chunk: &Span, start: usize) -> (TextSpan, usize) {
    let text_style = &text_styles[chunk.style_idx as usize];
    let fill = text_style.color.as_ref().map(|color| Fill {
        paint: usvg::Paint::Color(color.into()),
        opacity: color.opacity(),
        rule: Default::default(),
    });
    let font = Font {
        families: vec![text_style.font_family.as_ref().clone()],
        style: if text_style.italic {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        },
        stretch: text_style.stretch,
        weight: text_style.weight,
    };
    let decoration = TextDecoration {
        underline: None,
        overline: None,
        line_through: None,
    };
    let stroke = text_style
        .stroke
        .as_ref()
        .map(|s| stroke_to_usvg_stroke(s));
    let end = start + chunk.length as usize;
    (
        TextSpan {
            start,
            end,
            fill,
            stroke,
            paint_order: PaintOrder::FillAndStroke,
            font,
            font_size: NonZeroPositiveF32::new(text_style.size).unwrap(),
            small_caps: false,
            apply_kerning: false,
            decoration,
            dominant_baseline: DominantBaseline::Auto,
            alignment_baseline: AlignmentBaseline::Auto,
            baseline_shift: vec![],
            visibility: Visibility::Visible,
            letter_spacing: 0.0,
            word_spacing: 0.0,
            text_length: None,
            length_adjust: LengthAdjust::default(),
        },
        end,
    )
}

fn render_line(text_styles: &[TextStyle], styled_line: &StyledLine, x: f32, y: f32) -> TextChunk {
    let mut pos = 0;
    TextChunk {
        x: Some(x),
        y: Some(y),
        anchor: TextAnchor::Start,
        spans: styled_line
            .spans
            .iter()
            .map(|span| {
                let (span, new_pos) = create_svg_span(text_styles, span, pos);
                pos = new_pos;
                span
            })
            .collect(),
        text_flow: TextFlow::Linear,
        text: styled_line.text.clone(),
    }
}

pub(crate) fn render_text(styled_text: &StyledText, x: f32, y: f32) -> usvg::Node {
    let n_chars = styled_text
        .styled_lines
        .iter()
        .map(|sl| sl.text.len())
        .sum();
    let pos_list = vec![
        CharacterPosition {
            x: None,
            y: None,
            dx: None,
            dy: None,
        };
        n_chars
    ];
    let rot_list = vec![0.0; n_chars];

    let mut current_y = y;
    let chunks: Vec<TextChunk> = styled_text
        .styled_lines
        .iter()
        .map(|styled_line| {
            let line_height = styled_line
                .line_height(&styled_text.styles)
                .unwrap_or_else(|| styled_text.default_line_height());
            let font_size = styled_line
                .font_size(&styled_text.styles)
                .unwrap_or(styled_text.default_font_size);
            current_y += line_height;
            let half_space = (line_height - font_size) / 2.0;
            render_line(&styled_text.styles, styled_line, x, current_y - half_space)
        })
        .collect();
    let text = Text {
        id: String::new(),
        transform: Default::default(),
        rendering_mode: TextRendering::GeometricPrecision,
        positions: pos_list,
        rotate: rot_list,
        writing_mode: WritingMode::LeftToRight,
        chunks,
    };
    usvg::Node::new(NodeKind::Text(text))
}
