use super::node::Node;
use crate::model::{Color, Length, LengthOrAuto, NodeId, Step, StepValue};

#[derive(Debug)]
pub(crate) struct Slide {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) node: Node,
    pub(crate) n_steps: Step,
    pub(crate) name: String,
    node_id_counter: NodeId,
}

impl Slide {
    pub fn new(width: f32, height: f32, name: String, bg_color: Color) -> Self {
        Slide {
            width,
            height,
            node: Node {
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
                name: name.clone(),
                debug_layout: None,
            },
            n_steps: 1,
            name,
            node_id_counter: NodeId::new(0),
        }
    }
    pub fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter.bump()
    }
}

#[derive(Debug, Default)]
pub(crate) struct SlideDeck {
    pub(crate) slides: Vec<Slide>,
}
