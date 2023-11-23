use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum PathPart {
    Move {
        x: f32,
        y: f32,
    },
    Line {
        x: f32,
        y: f32,
    },
    Quad {
        x1: f32,
        x2: f32,
        x: f32,
        y: f32,
    },
    Cubic {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x: f32,
        y: f32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Path {
    parts: Vec<PathPart>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Drawing {
    paths: Vec<Path>,
}
