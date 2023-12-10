use crate::common::error::NelsieError;
use crate::model::LayoutExpr::ConstValue;
use std::fmt::Display;
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
    pub(crate) const ZERO: Length = Length::Points { value: 0.0 };
}

#[derive(Debug, Clone)]
pub(crate) enum LengthOrAuto {
    Points { value: f32 },
    Fraction { value: f32 },
    Auto,
}

impl LengthOrAuto {
    pub(crate) const ZERO: LengthOrAuto = LengthOrAuto::Points { value: 0.0 };
}

#[derive(Debug, Clone)]
pub(crate) enum LayoutExpr {
    ConstValue { value: f32 },
    ParentX { shift: f32 },
    ParentY { shift: f32 },
    ParentWidth { fraction: f32 },
    ParentHeight { fraction: f32 },
    X { node_id: NodeId },
    Y { node_id: NodeId },
    Width { node_id: NodeId, fraction: f32 },
    Height { node_id: NodeId, fraction: f32 },
    Sum { expressions: Vec<LayoutExpr> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl FromStr for Color {
    type Err = crate::NelsieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::new(svgtypes::Color::from_str(s).map_err(|_| {
            NelsieError::ParsingError("Invalid color".to_string())
        })?))
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0.red, self.0.green, self.0.blue)
    }
}

#[derive(Debug)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}
