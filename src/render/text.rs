use usvg::{fontdb, Color, NonZeroPositiveF32, TreeTextToPath};
use usvg_tree::{
    AlignmentBaseline, CharacterPosition, DominantBaseline, Fill, Font, FontStretch, FontStyle,
    LengthAdjust, NodeKind, PaintOrder, Text, TextAnchor, TextChunk, TextDecoration, TextFlow,
    TextRendering, TextSpan, Visibility, WritingMode,
};

pub(crate) fn get_text_size(text: &str) -> (f32, f32) {
    let text_node = render_text(text, 0.0, 0.0);
    let mut root_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));
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
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    tree.convert_text(&fontdb);
    let g = tree.root.first_child().unwrap().first_child().unwrap();
    let result = match *g.borrow() {
        NodeKind::Path(ref path) => {
            let bbox = path.text_bbox.unwrap();
            (bbox.width(), bbox.height())
        }
        _ => unreachable!(),
    };
    result
}

pub(crate) fn render_text(text: &str, x: f32, y: f32) -> usvg::Node {
    let n_chars = text.len();
    let mut pos_list = vec![
        CharacterPosition {
            x: None,
            y: None,
            dx: None,
            dy: None,
        }; n_chars];
    pos_list[0].x = Some(x);
    pos_list[0].y = Some(y);
    let rot_list = vec![0.0; n_chars];
    let fill = Fill {
        paint: usvg::Paint::Color(Color::black()),
        ..Default::default()
    };
    let fill2 = Fill {
        paint: usvg::Paint::Color(Color::new_rgb(0, 255, 0)),
        ..Default::default()
    };
    let span = TextSpan {
        start: 0,
        end: 6,
        fill: Some(fill),
        stroke: None,
        paint_order: PaintOrder::FillAndStroke,
        font: Font {
            families: vec!["Ubuntu".to_string()],
            style: FontStyle::Normal,
            stretch: FontStretch::Normal,
            weight: 400,
        },
        font_size: NonZeroPositiveF32::new(32.0).unwrap(),
        small_caps: false,
        apply_kerning: false,
        decoration: TextDecoration {
            underline: None,
            overline: None,
            line_through: None,
        },
        dominant_baseline: DominantBaseline::Auto,
        alignment_baseline: AlignmentBaseline::Auto,
        baseline_shift: vec![],
        visibility: Visibility::Visible,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_length: None,
        length_adjust: LengthAdjust::default(),
    };

    let span2 = TextSpan {
        start: 6,
        end: 11,
        fill: Some(fill2),
        stroke: None,
        paint_order: PaintOrder::FillAndStroke,
        font: Font {
            families: vec!["Ubuntu".to_string()],
            style: FontStyle::Normal,
            stretch: FontStretch::Normal,
            weight: 400,
        },
        font_size: NonZeroPositiveF32::new(48.0).unwrap(),
        small_caps: false,
        apply_kerning: false,
        decoration: TextDecoration {
            underline: None,
            overline: None,
            line_through: None,
        },
        dominant_baseline: DominantBaseline::Auto,
        alignment_baseline: AlignmentBaseline::Auto,
        baseline_shift: vec![],
        visibility: Visibility::Visible,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_length: None,
        length_adjust: LengthAdjust::default(),
    };

    let chunks = vec![TextChunk {
        x: Some(x),
        y: Some(y),
        anchor: TextAnchor::Start,
        spans: vec![span, span2],
        text_flow: TextFlow::Linear,
        text: text.to_string(),
    }];
    let text = Text {
        id: String::new(),
        transform: Default::default(),
        rendering_mode: TextRendering::GeometricPrecision,
        positions: pos_list,
        rotate: rot_list,
        writing_mode: WritingMode::LeftToRight,
        chunks,
    };
    usvg::Node::new(usvg::NodeKind::Text(text))
}
