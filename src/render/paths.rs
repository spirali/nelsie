use crate::common::Rectangle;
use crate::model::{NodeId, Path, PathPart, Stroke};
use crate::parsers::SimpleXmlWriter;
use crate::render::layout::ComputedLayout;
use crate::render::pathbuilder::{svg_ellipse, PathBuilder};

fn move_point_for_arrow(
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
    index: usize,
) -> Option<(f32, f32)> {
    if path.arrow_end.is_some() && index == path.parts.len() - 1 {
        let mut parts = path.parts.iter().rev();
        if let Some((_, _, dx, dy)) =
            arrow_direction(layout, parent_id, parts.next()?, parts.next())
        {
            let arrow = path.arrow_end.as_ref().unwrap();
            let len = if let Some(w) = arrow.stroke_width {
                w / 2.0
            } else {
                arrow.size * arrow.inner_point.unwrap_or(1.0) / 2.0
            };
            return Some((-dx * len, -dy * len));
        }
    }
    if path.arrow_start.is_some() && index == 0 {
        let mut parts = path.parts.iter();
        if let Some((_, _, dx, dy)) =
            arrow_direction(layout, parent_id, parts.next()?, parts.next())
        {
            let arrow = path.arrow_start.as_ref().unwrap();
            let len = if let Some(w) = arrow.stroke_width {
                w / 2.0
            } else {
                arrow.size * arrow.inner_point.unwrap_or(1.0) / 2.0
            };
            return Some((-dx * len, -dy * len));
        }
    }
    None
}

pub(crate) fn path_to_svg(
    xml: &mut SimpleXmlWriter,
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
) {
    if let Some(PathPart::Oval { x1, y1, x2, y2 }) = path.parts.first() {
        let x1 = layout.eval(x1, parent_id);
        let y1 = layout.eval(y1, parent_id);
        let x2 = layout.eval(x2, parent_id);
        let y2 = layout.eval(y2, parent_id);
        svg_ellipse(
            xml,
            &Rectangle::new(x1, y1, x2 - x1, y2 - y1),
            &path.stroke,
            &path.fill_color,
        );
    } else {
        let mut builder = PathBuilder::new(path.stroke.clone(), path.fill_color);
        for (i, part) in path.parts.iter().enumerate() {
            let (sx, sy) = move_point_for_arrow(layout, parent_id, path, i).unwrap_or((0.0, 0.0));
            match part {
                PathPart::Move { x, y } => {
                    builder.move_to(
                        layout.eval(x, parent_id) + sx,
                        layout.eval(y, parent_id) + sy,
                    );
                }
                PathPart::Line { x, y } => {
                    builder.line_to(
                        layout.eval(x, parent_id) + sx,
                        layout.eval(y, parent_id) + sy,
                    );
                }
                PathPart::Quad { x1, y1, x, y } => builder.quad_to(
                    layout.eval(x1, parent_id),
                    layout.eval(y1, parent_id),
                    layout.eval(x, parent_id),
                    layout.eval(y, parent_id),
                ),
                PathPart::Cubic {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => builder.cubic_to(
                    layout.eval(x1, parent_id),
                    layout.eval(y1, parent_id),
                    layout.eval(x2, parent_id),
                    layout.eval(y2, parent_id),
                    layout.eval(x, parent_id),
                    layout.eval(y, parent_id),
                ),
                PathPart::Close => builder.close(),
                PathPart::Oval { .. } => { /* Ignoring Oval, it has to be first if it used */ }
            }
        }
        builder.write_svg(xml);
    }
}

fn arrow_direction(
    layout: &ComputedLayout,
    parent_id: NodeId,
    part1: &PathPart,
    part2: Option<&PathPart>,
) -> Option<(f32, f32, f32, f32)> {
    let (x, y, dx, dy) = match part1 {
        PathPart::Move { x, y } | PathPart::Line { x, y } => {
            let part2 = part2?;
            let (x2, y2) = part2.main_point()?;
            let x = layout.eval(x, parent_id);
            let y = layout.eval(y, parent_id);
            let x2 = layout.eval(x2, parent_id);
            let y2 = layout.eval(y2, parent_id);
            (x, y, x - x2, y - y2)
        }
        PathPart::Quad {
            x1: x2,
            y1: y2,
            x,
            y,
        }
        | PathPart::Cubic { x2, y2, x, y, .. } => {
            let x = layout.eval(x, parent_id);
            let y = layout.eval(y, parent_id);
            let x2 = layout.eval(x2, parent_id);
            let y2 = layout.eval(y2, parent_id);
            (x, y, x - x2, y - y2)
        }
        PathPart::Close | PathPart::Oval { .. } => {
            return None;
        }
    };
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.0001 {
        None
    } else {
        Some((x, y, dx / len, dy / len))
    }
}

pub(crate) fn create_arrow(
    xml: &mut SimpleXmlWriter,
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
    is_end_arrow: bool,
) -> Option<()> {
    let arrow = if is_end_arrow {
        path.arrow_end.as_ref()?
    } else {
        path.arrow_start.as_ref()?
    };
    let color = arrow
        .color
        .as_ref()
        .or_else(|| path.stroke.as_ref().map(|s| &s.color))?;
    let (p1, p2) = if is_end_arrow {
        let mut parts = path.parts.iter().rev();
        (parts.next()?, parts.next())
    } else {
        let mut parts = path.parts.iter();
        (parts.next()?, parts.next())
    };
    let (x, y, dx, dy) = arrow_direction(layout, parent_id, p1, p2)?;
    let angle = arrow.angle * std::f32::consts::PI / 180.0;
    let a = dx.atan2(dy) + std::f32::consts::PI;
    let x1 = x + arrow.size * (a - angle).sin();
    let y1 = y + arrow.size * (a - angle).cos();
    let x2 = x + arrow.size * (a + angle).sin();
    let y2 = y + arrow.size * (a + angle).cos();

    let (stroke, fill_color) = if let Some(width) = arrow.stroke_width {
        (
            Some(Stroke {
                color: *color,
                width,
                dash_array: None,
                dash_offset: 0.0,
            }),
            None,
        )
    } else {
        (None, Some(*color))
    };
    let mut builder = PathBuilder::new(stroke, fill_color);
    builder.move_to(x1, y1);
    builder.line_to(x, y);
    builder.line_to(x2, y2);
    if arrow.stroke_width.is_none() {
        if let Some(inner) = arrow.inner_point {
            let x3 = x - arrow.size * inner * dx * angle.sin();
            let y3 = y - arrow.size * inner * dy * angle.cos();
            builder.line_to(x3, y3);
        }
        builder.close();
    }
    builder.write_svg(xml);
    Some(())
}

pub(crate) fn path_from_rect(path: &mut PathBuilder, rect: &Rectangle, border_radius: f32) {
    if border_radius < 0.001 {
        path.rect(rect)
    } else {
        let x2 = rect.x + rect.width;
        let y2 = rect.y + rect.height;
        path.move_to(rect.x + border_radius, rect.y);
        path.line_to(x2 - border_radius, rect.y);
        path.quad_to(x2, rect.y, x2, rect.y + border_radius);
        path.line_to(x2, y2 - border_radius);
        path.quad_to(x2, y2, x2 - border_radius, y2);
        path.line_to(rect.x + border_radius, y2);
        path.quad_to(rect.x, y2, rect.x, y2 - border_radius);
        path.line_to(rect.x, rect.y + border_radius);
        path.quad_to(rect.x, rect.y, rect.x + border_radius, rect.y);
    }
}
