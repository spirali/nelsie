use crate::common::{Color, Path, Stroke};

#[derive(Debug, Clone)]
pub(crate) struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DrawRect {
    pub rectangle: Rectangle,
    pub fill_color: Option<Color>,
    pub stroke: Option<Stroke>,
}

#[derive(Debug)]
pub(crate) enum DrawItem {
    Rect(DrawRect),
    Oval(DrawRect),
    Path(Path),
}
