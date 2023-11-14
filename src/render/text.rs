use crate::model::{Span, StyledLine, StyledText, TextStyle};
use usvg::{fontdb, NonZeroPositiveF32, TreeTextToPath};
use usvg_tree::{
    AlignmentBaseline, CharacterPosition, DominantBaseline, Fill, Font, FontStretch, FontStyle,
    LengthAdjust, NodeKind, PaintOrder, Text, TextAnchor, TextChunk, TextDecoration, TextFlow,
    TextRendering, TextSpan, Visibility, WritingMode,
};

pub(crate) fn get_text_size(font_db: &fontdb::Database, text: &StyledText) -> (f32, f32) {
    let text_node = render_text(text, 0.0, 0.0);
    let root_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));
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
        for line in &text.styled_lines {
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

fn create_svg_span(text_styles: &[TextStyle], chunk: &Span) -> TextSpan {
    let text_style = &text_styles[chunk.style_idx as usize];
    let fill = Fill {
        paint: usvg::Paint::Color((&text_style.color).into()),
        ..Default::default()
    };
    let font = Font {
        families: vec!["Ubuntu".to_string()],
        style: FontStyle::Normal,
        stretch: FontStretch::Normal,
        weight: 400,
    };
    let decoration = TextDecoration {
        underline: None,
        overline: None,
        line_through: None,
    };
    TextSpan {
        start: chunk.start as usize,
        end: chunk.start as usize + chunk.length as usize,
        fill: Some(fill),
        stroke: None,
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
    }
}

fn render_line(text_styles: &[TextStyle], styled_line: &StyledLine, x: f32, y: f32) -> TextChunk {
    TextChunk {
        x: Some(x),
        y: Some(y),
        anchor: TextAnchor::Start,
        spans: styled_line
            .spans
            .iter()
            .map(|span| create_svg_span(text_styles, span))
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
        }; n_chars];
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
