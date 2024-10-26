use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}

#[derive(Debug)]
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
pub(crate) struct Path {
    parts: Vec<PathPart>,
    fill_color: Option<Color>,
    stroke: Option<Stroke>,
}

impl Path {
    pub fn parts(&self) -> &[PathPart] {
        &self.parts
    }

    pub fn stroke(&self) -> &Option<Stroke> {
        &self.stroke
    }

    pub fn fill_color(&self) -> &Option<Color> {
        &self.fill_color
    }
}

pub(crate) struct PathBuilder(Path);

impl PathBuilder {
    pub fn new(stroke: Option<Stroke>, fill_color: Option<Color>) -> Self {
        PathBuilder(Path {
            parts: Vec::new(),
            fill_color,
            stroke,
        })
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.0.parts.push(PathPart::Move { x, y })
    }
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.0.parts.push(PathPart::Line { x, y })
    }
    pub fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.parts.push(PathPart::Quad { x1, y1, x, y })
    }
    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.parts.push(PathPart::Cubic {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        })
    }

    pub fn close(&mut self) {
        self.0.parts.push(PathPart::Close);
    }

    pub fn build(self) -> Path {
        self.0
    }
}
