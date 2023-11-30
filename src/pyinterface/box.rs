use pyo3::{FromPyObject, PyResult};
use crate::model::{Color, Length, LengthOrAuto, Node, NodeId, Slide, StepValue};
use crate::parsers::{parse_color, parse_length};
use crate::pyinterface::insteps::ValueOrInSteps;

#[derive(Debug, FromPyObject)]
pub(crate) enum StringOrFloat {
    Float(f32),
    String(String),
}

#[derive(Debug, FromPyObject)]
pub(crate) struct BoxConfig {
    pub bg_color: ValueOrInSteps<Option<String>>,
    pub width: ValueOrInSteps<Option<StringOrFloat>>,
    pub height: ValueOrInSteps<Option<StringOrFloat>>,
    pub name: String,
}

pub fn pyparse_opt_length(obj: Option<StringOrFloat>) -> crate::Result<Option<Length>> {
    match obj {
        None => Ok(None),
        Some(StringOrFloat::String(v)) => parse_length(&v).map(Some),
        Some(StringOrFloat::Float(value)) => Ok(Some(Length::Points { value }))
    }
}

impl BoxConfig {
    pub fn make_node(self, node_id: NodeId) -> PyResult<Node> {
        let bg_color = self.bg_color.parse(|v| v.as_deref().map(parse_color).transpose())?;
        let width = self.width.parse(pyparse_opt_length)?;
        let height = self.height.parse(pyparse_opt_length)?;

        Ok(Node {
            node_id,
            name: self.name,
            show: StepValue::new_const(true),
            z_level: StepValue::Const(0),
            x: StepValue::Const(None),
            y: StepValue::Const(None),
            width,
            height,
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
            bg_color,
            content: StepValue::Const(None),
            debug_layout: None,
            children: Vec::new(),
        })
    }
}