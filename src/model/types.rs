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
pub(crate) enum Size {
    Points { value: f32 },
    Fraction { value: f32 },
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

#[derive(Debug)]
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

// A conditionally-compiled module
#[cfg(test)]
mod test {
    use crate::model::LayoutExpr::{ConstValue, Sum, X};
    use crate::model::{LayoutExpr, NodeId};
}
