use super::node::Node;
use crate::common::error::NelsieError;
use crate::model::{
    Color, Length, LengthOrAuto, NodeId, PartialTextStyle, Resources, Step, StepValue, StyleMap,
};
use std::collections::HashMap;
use std::sync::Arc;
use usvg_tree::FontStretch;

#[derive(Debug)]
pub(crate) struct Slide {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) node: Node,
    pub(crate) n_steps: Step,
    node_id_counter: NodeId,
}

impl Slide {
    pub fn new(
        width: f32,
        height: f32,
        name: String,
        bg_color: Color,
        styles: Arc<StyleMap>,
    ) -> Self {
        Slide {
            width,
            height,
            node: Node {
                styles,
                name,
                node_id: NodeId::new(0),
                children: vec![],
                show: StepValue::new_const(true),
                z_level: StepValue::Const(0),
                x: StepValue::Const(None),
                y: StepValue::Const(None),
                width: StepValue::Const(Some(Length::Points { value: width })),
                height: StepValue::Const(Some(Length::Points { value: height })),
                row: StepValue::Const(false),
                reverse: StepValue::Const(false),
                p_top: StepValue::Const(Length::ZERO),
                p_bottom: StepValue::Const(Length::ZERO),
                p_left: StepValue::Const(Length::ZERO),
                p_right: StepValue::Const(Length::ZERO),
                m_top: StepValue::Const(LengthOrAuto::ZERO),
                m_bottom: StepValue::Const(LengthOrAuto::ZERO),
                m_left: StepValue::Const(LengthOrAuto::ZERO),
                m_right: StepValue::Const(LengthOrAuto::ZERO),
                bg_color: StepValue::Const(Some(bg_color)),
                content: StepValue::Const(None),
                debug_layout: None,
            },
            n_steps: 1,
            node_id_counter: NodeId::new(0),
        }
    }
    pub fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter.bump()
    }
}

#[derive(Debug)]
pub(crate) struct SlideDeck {
    pub(crate) slides: Vec<Slide>,
    pub(crate) global_styles: Arc<StyleMap>,
    pub(crate) default_font_family: Arc<String>,
}

impl SlideDeck {
    pub fn new(resources: &Resources, default_font: Option<&str>) -> crate::Result<Self> {
        let default_font_family = Arc::new(
            if let Some(font) = default_font {
                resources.check_font(font)?
            } else {
                ["DejaVu Sans", "Arial"]
                    .iter()
                    .find_map(|n| resources.check_font(n).ok())
                    .ok_or_else(|| {
                        NelsieError::Generic(
                        "No default font detected. Specify parameter 'default_font' in SlideDeck"
                            .to_string(),
                    )
                    })?
            }
            .to_string(),
        );
        let default_style = PartialTextStyle {
            font_family: Some(default_font_family.clone()),
            stroke: Some(None),
            color: Some(Some(Color::new(svgtypes::Color::black()))),
            size: Some(32.0),
            line_spacing: Some(1.2),
            italic: Some(false),
            stretch: Some(FontStretch::Normal),
            weight: Some(400),
        };
        let mut styles = HashMap::new();
        styles.insert("default".to_string(), StepValue::new_const(default_style));
        Ok(Self {
            slides: Vec::new(),
            global_styles: Arc::new(StyleMap::new(styles)),
            default_font_family,
        })
    }
}
