use crate::model::LayoutExpr::ConstValue;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct NodeId(u32);

impl NodeId {
    pub fn new(node_id: u32) -> Self {
        NodeId(node_id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn bump(&mut self) -> NodeId {
        self.0 += 1;
        NodeId::new(self.0)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum Length {
    Points { value: f32 },
    Fraction { value: f32 },
}

impl Length {
    pub(crate) const ZERO: Length = Length::Points {value: 0.0 };
}


#[derive(Debug, Clone)]
pub(crate) enum LengthOrAuto {
    Points { value: f32 },
    Fraction { value: f32 },
    Auto,
}

impl LengthOrAuto {
    pub(crate) const ZERO : LengthOrAuto = LengthOrAuto::Points { value: 0.0 };
}

#[derive(Debug, Clone)]
pub(crate) enum LayoutExpr {
    ConstValue { value: f32 },
    X { node_id: NodeId },
    Y { node_id: NodeId },
    Width { node_id: NodeId, fraction: f32 },
    Height { node_id: NodeId, fraction: f32 },
    Sum { expressions: Vec<LayoutExpr> },
}

#[derive(Debug, Clone)]
pub(crate) struct Color(svgtypes::Color);

impl Color {
    pub fn new(color: svgtypes::Color) -> Self {
        Color(color)
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}