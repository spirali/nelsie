use crate::model::{LayoutExpr, StepValue, Stroke};

#[derive(Debug)]
pub(crate) enum PathPart {
    Move {
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Line {
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Quad {
        x1: LayoutExpr,
        y1: LayoutExpr,
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Cubic {
        x1: LayoutExpr,
        y1: LayoutExpr,
        x2: LayoutExpr,
        y2: LayoutExpr,
        x: LayoutExpr,
        y: LayoutExpr,
    },
}

#[derive(Debug)]
pub(crate) struct Path {
    pub stroke: Option<Stroke>,
    pub parts: Vec<PathPart>,
}

#[derive(Debug)]
pub(crate) struct Drawing {
    pub paths: StepValue<Vec<Path>>,
}
