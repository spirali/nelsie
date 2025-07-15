use crate::Rectangle;
use crate::shapes::FillAndStroke;

#[derive(Debug)]
pub(crate) struct DrawRect {
    pub rectangle: Rectangle,
    pub fill_and_stroke: FillAndStroke,
}

#[derive(Debug)]
pub(crate) enum DrawItem {
    Rect(DrawRect),
    Oval(DrawRect),
    Path(DrawPath),
}

#[derive(Debug)]
pub(crate) enum DrawPathPart {
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
        y1: f32,
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
    Close,
}

#[derive(Debug)]
pub(crate) struct DrawPath {
    parts: Vec<DrawPathPart>,
    fill_and_stroke: FillAndStroke,
}

impl DrawPath {
    pub fn parts(&self) -> &[DrawPathPart] {
        &self.parts
    }

    pub fn fill_and_stroke(&self) -> &FillAndStroke {
        &self.fill_and_stroke
    }
}

pub(crate) struct PathBuilder(DrawPath);

impl PathBuilder {
    pub fn new(fill_and_stroke: FillAndStroke) -> Self {
        PathBuilder(DrawPath {
            parts: Vec::new(),
            fill_and_stroke,
        })
    }

    pub fn last_point(&self) -> (f32, f32) {
        match self.0.parts.last() {
            Some(DrawPathPart::Move { x, y })
            | Some(DrawPathPart::Line { x, y })
            | Some(DrawPathPart::Quad { x, y, .. })
            | Some(DrawPathPart::Cubic { x, y, .. }) => (*x, *y),
            None | Some(DrawPathPart::Close) => (0.0, 0.0),
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.0.parts.push(DrawPathPart::Move { x, y })
    }
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.0.parts.push(DrawPathPart::Line { x, y })
    }
    pub fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.parts.push(DrawPathPart::Quad { x1, y1, x, y })
    }
    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.parts.push(DrawPathPart::Cubic {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        })
    }

    pub fn close(&mut self) {
        self.0.parts.push(DrawPathPart::Close);
    }

    pub fn build(self) -> DrawPath {
        self.0
    }
}
