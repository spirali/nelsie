use crate::model::{Resources, Span, StyledLine, StyledText, TextAlign, TextStyle};

use crate::render::layout::{Rectangle, TextLayout};
use crate::render::paths::stroke_to_usvg_stroke;
use usvg::{NonZeroPositiveF32, TreeTextToPath};
use usvg_tree::{
    AlignmentBaseline, CharacterPosition, DominantBaseline, Fill, Font, FontStyle, LengthAdjust,
    NodeKind, PaintOrder, Text, TextAnchor, TextChunk, TextDecoration, TextFlow, TextRendering,
    TextSpan, Visibility, WritingMode,
};

pub(crate) fn get_text_layout(
    resources: &Resources,
    text: &StyledText,
    align: TextAlign,
) -> (f32, f32, TextLayout) {
    let mut tmp_text = StyledText {
        styled_lines: &[],
        styles: text.styles.clone(),
        default_font_size: text.default_font_size,
        default_line_spacing: text.default_line_spacing,
    };
    let mut result = TextLayout {
        lines: Vec::with_capacity(text.styled_lines.len()),
    };
    let mut current_y = 0.0;
    for idx in 0..text.styled_lines.len() {
        tmp_text.styled_lines = &text.styled_lines[idx..idx + 1];
        let sx = get_text_width(resources, &tmp_text);
        let size = text.styled_lines[idx]
            .font_size(&tmp_text.styles)
            .unwrap_or(tmp_text.default_font_size);

        result.lines.push(Rectangle {
            x: 0.0,
            y: current_y,
            width: sx,
            height: size,
        });
        current_y += size * tmp_text.default_line_spacing;
    }

    let width = result
        .lines
        .iter()
        .map(|line| line.width)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(0.0);
    (width, text.height(), result)
}

fn get_text_width(resources: &Resources, text: &StyledText) -> f32 {
    let text_node = render_text(text, 0.0, 0.0, TextAlign::Start);
    let root_node = usvg::Node::new(NodeKind::Group(usvg::Group::default()));
    root_node.append(text_node);
    let size = usvg::Size::from_wh(8000.0, 6000.0).unwrap();
    let mut tree = usvg_tree::Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_node,
    };
    tree.convert_text(&resources.font_db);
    //let mut x1 = f32::MAX;
    let mut x2 = f32::MIN;
    if let Some(main) = tree.root.first_child() {
        for child in main.children() {
            let borrowed = child.borrow();
            match *borrowed {
                NodeKind::Path(ref path) => {
                    let bbox = path.text_bbox.unwrap();
                    //x1 = x1.min(bbox.left());
                    x2 = x2.max(bbox.right());
                }
                _ => unreachable!(),
            }
        }
    }
    let mut width = x2; // - x1;
    if !f32::is_finite(width) || width < 0.0 {
        width = 0.0;
    }
    width
}

fn create_svg_span(text_styles: &[TextStyle], chunk: &Span, start: usize) -> (TextSpan, usize) {
    let text_style = &text_styles[chunk.style_idx as usize];
    let fill = text_style.color.as_ref().map(|color| Fill {
        paint: usvg::Paint::Color(color.into()),
        opacity: color.opacity(),
        rule: Default::default(),
    });
    let font = Font {
        families: vec![text_style.font.family_name.clone()],
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
    let stroke = text_style.stroke.as_ref().map(|s| stroke_to_usvg_stroke(s));
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

fn render_line(
    text_styles: &[TextStyle],
    styled_line: &StyledLine,
    x: f32,
    y: f32,
    anchor: TextAnchor,
) -> TextChunk {
    let mut pos = 0;
    TextChunk {
        x: Some(x),
        y: Some(y),
        anchor,
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

pub(crate) fn render_text(
    styled_text: &StyledText,
    x: f32,
    y: f32,
    align: TextAlign,
) -> usvg::Node {
    let anchor = match align {
        TextAlign::Start => TextAnchor::Start,
        TextAlign::Center => TextAnchor::Middle,
        TextAlign::End => TextAnchor::End,
    };
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
    // let last = styled_text.stlen() - 1;
    let chunks: Vec<TextChunk> = styled_text
        .styled_lines
        .iter()
        .enumerate()
        .map(|(idx, styled_line)| {
            let size = styled_line
                .font_size(&styled_text.styles)
                .unwrap_or(styled_text.default_font_size);

            let height = if idx != 0 {
                size * styled_text.default_line_spacing
            } else {
                size
            };
            let descender = styled_line
                .line_descender(&styled_text.styles)
                .unwrap_or(0.0);
            current_y += height;
            render_line(
                &styled_text.styles,
                styled_line,
                x,
                current_y + descender,
                anchor,
            )
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
