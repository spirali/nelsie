use crate::common::error::NelsieError;
use std::fmt::{Display, Formatter};

use crate::model::InTextAnchorId;
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

impl Default for Length {
    fn default() -> Self {
        Length::Points { value: 0.0f32 }
    }
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

impl Default for LengthOrAuto {
    fn default() -> Self {
        LengthOrAuto::Points { value: 0.0f32 }
    }
}

impl LengthOrAuto {
    pub(crate) const ZERO: LengthOrAuto = LengthOrAuto::Points { value: 0.0 };
}

#[derive(Debug, Clone)]
pub(crate) enum LengthOrExpr {
    Points { value: f32 },
    Fraction { value: f32 },
    Expr(LayoutExpr),
}

impl Default for LengthOrExpr {
    fn default() -> Self {
        LengthOrExpr::Points { value: 0.0f32 }
    }
}

impl LengthOrExpr {
    pub fn is_expr(&self) -> bool {
        match self {
            LengthOrExpr::Points { .. } | LengthOrExpr::Fraction { .. } => false,
            LengthOrExpr::Expr(_) => true,
        }
    }

    pub fn as_expr(&self) -> Option<&LayoutExpr> {
        match self {
            LengthOrExpr::Points { .. } | LengthOrExpr::Fraction { .. } => None,
            LengthOrExpr::Expr(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LayoutExpr {
    ConstValue {
        value: f32,
    },
    ParentX {
        shift: f32,
    },
    ParentY {
        shift: f32,
    },
    ParentWidth {
        fraction: f32,
    },
    ParentHeight {
        fraction: f32,
    },
    X {
        node_id: NodeId,
    },
    Y {
        node_id: NodeId,
    },
    Width {
        node_id: NodeId,
        fraction: f32,
    },
    Height {
        node_id: NodeId,
        fraction: f32,
    },
    LineX {
        node_id: NodeId,
        line_idx: u32,
    },
    LineY {
        node_id: NodeId,
        line_idx: u32,
    },
    LineWidth {
        node_id: NodeId,
        line_idx: u32,
        fraction: f32,
    },
    LineHeight {
        node_id: NodeId,
        line_idx: u32,
        fraction: f32,
    },
    InTextAnchorX {
        node_id: NodeId,
        anchor_id: InTextAnchorId,
    },
    InTextAnchorY {
        node_id: NodeId,
        anchor_id: InTextAnchorId,
    },
    InTextAnchorWidth {
        node_id: NodeId,
        anchor_id: InTextAnchorId,
        fraction: f32,
    },
    InTextAnchorHeight {
        node_id: NodeId,
        anchor_id: InTextAnchorId,
        fraction: f32,
    },
    Sum {
        expressions: Vec<LayoutExpr>,
    },
}

impl LayoutExpr {
    pub(crate) fn add(self, other: LayoutExpr) -> LayoutExpr {
        LayoutExpr::Sum {
            expressions: vec![self, other],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Color(svgtypes::Color);

impl Color {
    pub fn new(color: svgtypes::Color) -> Self {
        Color(color)
    }

    pub fn as_3f32(&self) -> (f32, f32, f32) {
        (
            self.0.red as f32 / 255.0,
            self.0.green as f32 / 255.0,
            self.0.blue as f32 / 255.0,
        )
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}

impl FromStr for Color {
    type Err = NelsieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::new(svgtypes::Color::from_str(s).map_err(|_| {
            NelsieError::Parsing(format!("Invalid color: '{s}'"))
        })?))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.alpha == 255 {
            write!(
                f,
                "#{:02x}{:02x}{:02x}",
                self.0.red, self.0.green, self.0.blue
            )
        } else {
            write!(
                f,
                "#{:02x}{:02x}{:02x}{:02x}",
                self.0.red, self.0.green, self.0.blue, self.0.alpha
            )
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}
