use crate::render::draw::{DrawPath, DrawPathPart, DrawRect};
use crate::shapes::FillAndStroke;
use crate::utils::sxml::SimpleXmlWriter;
use std::fmt::Write;

pub(crate) fn stroke_and_fill_svg(xml: &mut SimpleXmlWriter, fill_and_stroke: &FillAndStroke) {
    if let Some(color) = &fill_and_stroke.fill_color {
        xml.attr("fill", color);
    } else {
        xml.attr("fill", "none");
    }
    if let Some(stroke) = &fill_and_stroke.stroke {
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

pub fn svg_path(xml: &mut SimpleXmlWriter, path: &DrawPath) {
    xml.begin("path");

    xml.attr_buf("d", |s| {
        for (i, part) in path.parts().iter().enumerate() {
            if i != 0 {
                s.push(' ');
            }
            match part {
                DrawPathPart::Move { x, y } => {
                    write!(s, "M {x} {y}").unwrap();
                }
                DrawPathPart::Line { x, y } => {
                    write!(s, "L {x} {y}").unwrap();
                }
                DrawPathPart::Quad { x1, y1, x, y } => write!(s, "Q {x1} {y1},{x} {y}").unwrap(),
                DrawPathPart::Cubic {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => write!(s, "C {x1} {y1},{x2} {y2},{x} {y}").unwrap(),
                DrawPathPart::Close => s.push('Z'),
            }
        }
    });
    stroke_and_fill_svg(xml, path.fill_and_stroke());
    xml.end("path");
}

pub(crate) fn svg_rect(xml: &mut SimpleXmlWriter, rect: &DrawRect) {
    xml.begin("rect");
    xml.attr("x", rect.rectangle.x);
    xml.attr("y", rect.rectangle.y);
    xml.attr("width", rect.rectangle.width);
    xml.attr("height", rect.rectangle.height);
    stroke_and_fill_svg(xml, &rect.fill_and_stroke);
    xml.end("rect");
}

pub(crate) fn svg_ellipse(xml: &mut SimpleXmlWriter, rect: &DrawRect) {
    let wh = rect.rectangle.width / 2.0;
    let hh = rect.rectangle.height / 2.0;
    xml.begin("ellipse");
    xml.attr("cx", rect.rectangle.x + wh);
    xml.attr("cy", rect.rectangle.y + hh);
    xml.attr("rx", wh);
    xml.attr("ry", hh);
    stroke_and_fill_svg(xml, &rect.fill_and_stroke);
    xml.end("ellipse");
}
