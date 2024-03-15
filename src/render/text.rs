use crate::model::{
    InTextAnchor, InTextAnchorId, InTextAnchorPoint, Resources, Span, StyledLine, StyledText,
    TextAlign, TextStyle,
};

use std::collections::HashMap;

use crate::render::layout::{Rectangle, TextLayout};
use crate::render::paths::stroke_to_usvg_stroke;
use svg2pdf::usvg;
use usvg::{
    AlignmentBaseline, DominantBaseline, Fill, Font, FontStyle, LengthAdjust, NonZeroPositiveF32,
    PaintOrder, PostProcessingSteps, Text, TextAnchor, TextChunk, TextDecoration, TextFlow,
    TextRendering, TextSpan, Tree, TreePostProc, Visibility, WritingMode,
};

pub(crate) fn get_in_text_anchor_point(text: &StyledText, point: &InTextAnchorPoint) -> StyledText {
    let line = &text.styled_lines[point.line_idx as usize];
    StyledText {
        styled_lines: vec![StyledLine {
            spans: line.spans[..point.span_idx as usize].to_vec(),
            text: line.text.to_string(),
        }],
        styles: text.styles.clone(),
        default_font_size: text.default_font_size,
        default_line_spacing: text.default_line_spacing,
    }
}

pub(crate) fn get_text_layout(
    resources: &Resources,
    text: &StyledText,
    align: TextAlign,
    anchors: &HashMap<InTextAnchorId, InTextAnchor>,
) -> (f32, f32, TextLayout) {
    let mut anchor_pos = HashMap::new();
    for anchor in anchors.values() {
        if !anchor_pos.contains_key(&anchor.start) {
            let sx = get_text_width(resources, &get_in_text_anchor_point(text, &anchor.start));
            anchor_pos.insert(anchor.start.clone(), sx);
        }
        if !anchor_pos.contains_key(&anchor.end) {
            let sx = get_text_width(resources, &get_in_text_anchor_point(text, &anchor.end));
            anchor_pos.insert(anchor.end.clone(), sx);
        }
    }

    let mut result_lines = Vec::with_capacity(text.styled_lines.len());

    let mut tmp_text = StyledText {
        styled_lines: Vec::new(),
        styles: text.styles.clone(),
        default_font_size: text.default_font_size,
        default_line_spacing: text.default_line_spacing,
    };

    let mut current_y = 0.0;
    for idx in 0..text.styled_lines.len() {
        tmp_text.styled_lines = vec![text.styled_lines[idx].clone()];
        let sx = get_text_width(resources, &tmp_text);
        let styled_line = &text.styled_lines[idx];
        let size = styled_line
            .font_size(&tmp_text.styles)
            .unwrap_or(tmp_text.default_font_size);
        let descender = if idx == 0 {
            0.0
        } else {
            styled_line.line_descender(&tmp_text.styles).unwrap_or(0.0)
        };

        result_lines.push(Rectangle {
            x: 0.0,
            y: current_y - descender,
            width: sx,
            height: size,
        });
        current_y += if idx == 0 {
            size
        } else {
            size * tmp_text.default_line_spacing
        }
    }

    let width = result_lines
        .iter()
        .map(|line| line.width)
        .max_by(|w1, w2| w1.partial_cmp(w2).unwrap())
        .unwrap_or(0.0);

    match align {
        TextAlign::Start => { /* Do nothing */ }
        TextAlign::Center => {
            for line in result_lines.iter_mut() {
                line.x = (width - line.width) / 2.0;
            }
        }
        TextAlign::End => {
            for line in result_lines.iter_mut() {
                line.x = width - line.width;
            }
        }
    }

    let anchor_points = anchors
        .iter()
        .map(|(anchor_id, anchor)| {
            let x = anchor_pos.get(&anchor.start).copied().unwrap();
            let start_line = &result_lines[anchor.start.line_idx as usize];
            (
                *anchor_id,
                Rectangle {
                    x: start_line.x + x,
                    y: start_line.y,
                    width: anchor_pos.get(&anchor.end).copied().unwrap() - x,
                    height: {
                        let end_line = &result_lines[anchor.end.line_idx as usize];
                        end_line.y + end_line.height - start_line.y
                    },
                },
            )
        })
        .collect();

    let result = TextLayout {
        lines: result_lines,
        anchor_points,
    };

    (width, text.height(), result)
}

fn get_text_width(resources: &Resources, text: &StyledText) -> f32 {
    assert_eq!(text.styled_lines.len(), 1);
    let text_node = render_text(text, 0.0, 0.0, TextAlign::Start);
    let mut root_node = usvg::Group::default();
    root_node.children.push(text_node);
    let size = usvg::Size::from_wh(8000.0, 6000.0).unwrap();
    let mut tree = Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_node,
    };
    let postprocessing = PostProcessingSteps {
        convert_text_into_paths: true,
    };
    tree.postprocess(postprocessing, &resources.font_db);

    let mut width = tree
        .root
        .abs_bounding_box()
        .map(|bbox| bbox.right())
        .unwrap_or(0.0);

    /* Because bounding box ignores span that contains only spaces, we have problem with trailing spaces if they are in separate style,
       we need to increase with for each trailing space
    */
    let line = &text.styled_lines[0];
    let line_len = line.spans.iter().map(|s| s.length as usize).sum();
    let mut line_chars = line.text.chars();
    for _ in line_len..line.text.len() {
        line_chars.next_back();
    }
    for span in line.spans.iter().rev() {
        if (0..span.length).all(|_| line_chars.next_back() == Some(' ')) {
            let style = &text.styles[span.style_idx as usize];
            width += style.font.space_size * style.size * span.length as f32;
        } else {
            break;
        }
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
            apply_kerning: text_style.kerning,
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
        rendering_mode: TextRendering::GeometricPrecision,
        dx: vec![],
        dy: vec![],
        rotate: rot_list,
        writing_mode: WritingMode::LeftToRight,
        chunks,
        abs_transform: Default::default(),
        bounding_box: None,
        stroke_bounding_box: None,
        flattened: None,
    };
    usvg::Node::Text(Box::new(text))
}
