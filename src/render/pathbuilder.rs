use crate::common::Rectangle;
use crate::model::{Color, Stroke};
use crate::parsers::SimpleXmlWriter;
use std::fmt::Write;

#[derive(Debug)]
enum PathPart {
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

pub(crate) fn stroke_and_fill_svg(
    xml: &mut SimpleXmlWriter,
    stroke: &Option<Stroke>,
    fill_color: &Option<Color>,
) {
    if let Some(color) = fill_color {
        xml.attr("fill", color);
    } else {
        xml.attr("fill", "none");
    }
    if let Some(stroke) = stroke {
        xml.attr("stroke", stroke.color);
        xml.attr("stroke-width", stroke.width);
        if let Some(array) = &stroke.dash_array {
            xml.attr_buf("stroke-dasharray", |s| {
                for (i, v) in array.iter().enumerate() {
                    if i == 0 {
                        write!(s, "{}", v).unwrap();
                    } else {
                        write!(s, " {}", v).unwrap();
                    }
                }
            });
            if stroke.dash_offset != 0.0 {
                xml.attr("stroke-dashoffset", stroke.dash_offset);
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PathBuilder {
    stroke: Option<Stroke>,
    fill_color: Option<Color>,
    parts: Vec<PathPart>,
}

impl PathBuilder {
    pub fn new(stroke: Option<Stroke>, fill_color: Option<Color>) -> Self {
        PathBuilder {
            stroke,
            fill_color,
            parts: Vec::new(),
        }
    }

    pub fn rect(&mut self, rect: &Rectangle) {
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;
        self.move_to(rect.x, rect.y);
        self.line_to(x2, rect.y);
        self.line_to(x2, y2);
        self.line_to(rect.x, y2);
        self.close();
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.parts.push(PathPart::Move { x, y })
    }
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.parts.push(PathPart::Line { x, y })
    }
    pub fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.parts.push(PathPart::Quad { x1, y1, x, y })
    }
    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.parts.push(PathPart::Cubic {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        })
    }

    pub fn close(&mut self) {
        self.parts.push(PathPart::Close);
    }

    pub fn write_svg(self, xml: &mut SimpleXmlWriter) {
        xml.begin("path");

        xml.attr_buf("d", |s| {
            for (i, part) in self.parts.iter().enumerate() {
                if i != 0 {
                    s.push(' ');
                }
                match part {
                    PathPart::Move { x, y } => {
                        write!(s, "M {x} {y}").unwrap();
                    }
                    PathPart::Line { x, y } => {
                        write!(s, "L {x} {y}").unwrap();
                    }
                    PathPart::Quad { x1, y1, x, y } => write!(s, "Q {x1} {y1},{x} {y}").unwrap(),
                    PathPart::Cubic {
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => write!(s, "C {x1} {y1},{x2} {y2},{x} {y}").unwrap(),
                    PathPart::Close => s.push('Z'),
                }
            }
        });
        stroke_and_fill_svg(xml, &self.stroke, &self.fill_color);
        xml.end("path");
    }
}

pub fn svg_ellipse(
    xml: &mut SimpleXmlWriter,
    rect: &Rectangle,
    stroke: &Option<Stroke>,
    fill_color: &Option<Color>,
) {
    let wh = rect.width / 2.0;
    let hh = rect.height / 2.0;
    xml.begin("ellipse");
    xml.attr("cx", rect.x + wh);
    xml.attr("cy", rect.y + hh);
    xml.attr("rx", wh);
    xml.attr("ry", hh);
    stroke_and_fill_svg(xml, stroke, fill_color);
    xml.end("ellipse");
}
