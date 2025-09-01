use crate::text::InlineId;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(node_id: usize) -> Self {
        NodeId(node_id)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }
    //
    // pub fn bump(&mut self) -> NodeId {
    //     self.0 += 1;
    //     NodeId::new(self.0)
    // }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Length {
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

#[derive(Debug, Copy, Clone)]
pub enum LengthOrAuto {
    Length(Length),
    Auto,
}

impl Default for LengthOrAuto {
    fn default() -> Self {
        LengthOrAuto::ZERO
    }
}

impl LengthOrAuto {
    pub(crate) const ZERO: LengthOrAuto = LengthOrAuto::Length(Length::Points { value: 0.0 });
}

#[derive(Debug, Clone)]
pub enum LengthOrExpr {
    Length(Length),
    Expr(LayoutExpr),
}

impl Default for LengthOrExpr {
    fn default() -> Self {
        LengthOrExpr::Length(Length::ZERO)
    }
}

impl LengthOrExpr {
    #[inline]
    pub fn points(value: f32) -> LengthOrExpr {
        LengthOrExpr::Length(Length::Points { value })
    }

    pub fn is_expr(&self) -> bool {
        match self {
            LengthOrExpr::Length(_) => false,
            LengthOrExpr::Expr(_) => true,
        }
    }

    pub fn as_expr(&self) -> Option<&LayoutExpr> {
        match self {
            LengthOrExpr::Length(_) => None,
            LengthOrExpr::Expr(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LayoutExpr {
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
    InlineX {
        node_id: NodeId,
        inline_id: InlineId,
    },
    InlineY {
        node_id: NodeId,
        inline_id: InlineId,
    },
    InlineWidth {
        node_id: NodeId,
        inline_id: InlineId,
        fraction: f32,
    },
    InlineHeight {
        node_id: NodeId,
        inline_id: InlineId,
        fraction: f32,
    },
    Add {
        expressions: Box<(LayoutExpr, LayoutExpr)>,
    },
    Sub {
        expressions: Box<(LayoutExpr, LayoutExpr)>,
    },
    Mul {
        expressions: Box<(LayoutExpr, LayoutExpr)>,
    },
    Max {
        expressions: Vec<LayoutExpr>,
    },
}

#[allow(clippy::should_implement_trait)]
impl LayoutExpr {
    pub const ZERO: LayoutExpr = LayoutExpr::ConstValue { value: 0.0 };

    #[inline]
    pub fn const_value(value: f32) -> LayoutExpr {
        LayoutExpr::ConstValue { value }
    }

    #[inline]
    pub fn add(self, other: LayoutExpr) -> LayoutExpr {
        LayoutExpr::Add {
            expressions: Box::new((self, other)),
        }
    }

    #[inline]
    pub fn max(expressions: Vec<LayoutExpr>) -> LayoutExpr {
        LayoutExpr::Max { expressions }
    }

    #[inline]
    pub fn sub(self, other: LayoutExpr) -> LayoutExpr {
        LayoutExpr::Sub {
            expressions: Box::new((self, other)),
        }
    }

    #[inline]
    pub fn mul(self, other: LayoutExpr) -> LayoutExpr {
        LayoutExpr::Mul {
            expressions: Box::new((self, other)),
        }
    }
}
