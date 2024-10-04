use crate::common::{Color, Path, PathPart, Rectangle, Stroke};
use crate::parsers::SimpleXmlWriter;
use std::fmt::Write;

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

impl Path {
    pub fn write_svg(&self, xml: &mut SimpleXmlWriter) {
        xml.begin("path");

        xml.attr_buf("d", |s| {
            for (i, part) in self.parts().iter().enumerate() {
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
        stroke_and_fill_svg(xml, self.stroke(), self.fill_color());
        xml.end("path");
    }
}

pub(crate) fn svg_ellipse(
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
