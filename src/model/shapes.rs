use crate::model::{Color, LayoutExpr, StepValue, Stroke};

#[derive(Debug)]
pub(crate) struct Arrow {
    pub size: f32,
    pub angle: f32,
    pub color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub inner_point: Option<f32>,
}

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
    Close,

    // We should create Conic element, unfortunately, tiny skia does not expose conic_to as a public interface
    // So this is just a hack how to draw a circle
    Oval {
        x1: LayoutExpr,
        y1: LayoutExpr,
        x2: LayoutExpr,
        y2: LayoutExpr,
    },
}

impl PathPart {
    pub fn main_point(&self) -> Option<(&LayoutExpr, &LayoutExpr)> {
        match self {
            PathPart::Move { x, y }
            | PathPart::Line { x, y }
            | PathPart::Quad { x, y, .. }
            | PathPart::Cubic { x, y, .. } => Some((x, y)),
            PathPart::Close | PathPart::Oval { .. } => None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Path {
    pub stroke: Option<Stroke>,
    pub fill_color: Option<Color>,
    pub parts: Vec<PathPart>,
    pub arrow_start: Option<Arrow>,
    pub arrow_end: Option<Arrow>,
}

#[derive(Debug)]
pub(crate) struct Drawing {
    pub paths: StepValue<Vec<Path>>,
}
