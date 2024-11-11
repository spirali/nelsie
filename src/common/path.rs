use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct FillAndStroke {
    pub fill_color: Option<Color>,
    pub stroke: Option<Stroke>,
}

impl FillAndStroke {
    pub fn new_fill(color: Color) -> Self {
        FillAndStroke {
            fill_color: Some(color),
            stroke: None,
        }
    }
    pub fn new_stroke(stroke: Stroke) -> Self {
        FillAndStroke {
            fill_color: None,
            stroke: Some(stroke),
        }
    }
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
    fill_and_stroke: FillAndStroke,
}

impl Path {
    pub fn parts(&self) -> &[PathPart] {
        &self.parts
    }

    pub fn fill_and_stroke(&self) -> &FillAndStroke {
        &self.fill_and_stroke
    }
}

pub(crate) struct PathBuilder(Path);

impl PathBuilder {
    pub fn new(fill_and_stroke: FillAndStroke) -> Self {
        PathBuilder(Path {
            parts: Vec::new(),
            fill_and_stroke,
        })
    }

    pub fn last_point(&self) -> (f32, f32) {
        match self.0.parts.last() {
            Some(PathPart::Move { x, y })
            | Some(PathPart::Line { x, y })
            | Some(PathPart::Quad { x, y, .. })
            | Some(PathPart::Cubic { x, y, .. }) => (*x, *y),
            None | Some(PathPart::Close) => (0.0, 0.0),
        }
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

    #[allow(clippy::too_many_arguments)]
    pub fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    ) {
        let (last_x, last_y) = self.last_point();

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(last_x as f64, last_y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(rx as f64, ry as f64),
            x_rotation: (x_axis_rotation as f64).to_radians(),
            large_arc,
            sweep,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.cubic_to(
                        p1.x as f32,
                        p1.y as f32,
                        p2.x as f32,
                        p2.y as f32,
                        p.x as f32,
                        p.y as f32,
                    );
                });
            }
            None => {
                self.line_to(x, y);
            }
        }
    }

    pub fn close(&mut self) {
        self.0.parts.push(PathPart::Close);
    }

    pub fn build(self) -> Path {
        self.0
    }
}
