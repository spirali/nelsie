use crate::model::{Color, LayoutExpr, Stroke};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
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

#[derive(Debug, Deserialize)]
pub(crate) struct Path {
    pub stroke: Option<Stroke>,
    pub parts: Vec<PathPart>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Drawing {
    pub paths: Vec<Path>,
}
