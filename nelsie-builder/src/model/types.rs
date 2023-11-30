use crate::model::LayoutExpr::ConstValue;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct NodeId(u32);

impl NodeId {
    #[cfg(test)]
    pub fn new(node_id: u32) -> Self {
        NodeId(node_id)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum Length {
    Points { value: f32 },
    Fraction { value: f32 },
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum LengthOrAuto {
    Points { value: f32 },
    Fraction { value: f32 },
    Auto,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
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

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let color = svgtypes::Color::from_str(&value)
            .map_err(|_| serde::de::Error::custom("Invalid color"))?;
        Ok(Color(color))
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}


pub(crate) trait DefaultInstance {
    const DEFAULT: Self;

    fn default_instance_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

impl DefaultInstance for f32 {
    const DEFAULT: Self = 0.0;
}

impl DefaultInstance for i32 {
    const DEFAULT: Self = 0;
}

impl DefaultInstance for bool {
    const DEFAULT: Self = false;
}

impl DefaultInstance for Length {
    const DEFAULT: Self = Length::Points { value: 0.0 };
}

impl DefaultInstance for LengthOrAuto {
    const DEFAULT: Self = LengthOrAuto::Points { value: 0.0 };
}

impl<T> DefaultInstance for Option<T> {
    const DEFAULT: Self = None;
}

impl<T> DefaultInstance for Vec<T> {
    const DEFAULT: Self = Vec::new();
}
