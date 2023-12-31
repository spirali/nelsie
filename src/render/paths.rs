use crate::model::{NodeId, Path, PathPart, Stroke};
use crate::render::layout::ComputedLayout;
use resvg::tiny_skia::PathBuilder;
use std::rc::Rc;
use usvg::NonZeroPositiveF32;
use usvg_tree::Fill;

pub(crate) fn stroke_to_usvg_stroke(stroke: &Stroke) -> usvg::Stroke {
    usvg::Stroke {
        paint: usvg::Paint::Color((&stroke.color).into()),
        dasharray: stroke.dash_array.clone(),
        dashoffset: stroke.dash_offset,
        miterlimit: Default::default(),
        opacity: stroke.color.opacity(),
        width: NonZeroPositiveF32::new(stroke.width).unwrap(),
        linecap: Default::default(),
        linejoin: Default::default(),
    }
}

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

pub(crate) fn create_path(
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
) -> Option<usvg::Path> {
    let mut builder = PathBuilder::new();

    for (i, part) in path.parts.iter().enumerate() {
        let (sx, sy) = move_point_for_arrow(layout, parent_id, &path, i).unwrap_or((0.0, 0.0));
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
        }
    }
    builder.finish().map(|p| {
        let mut svg_path = usvg::Path::new(Rc::new(p));
        if let Some(stroke) = &path.stroke {
            svg_path.stroke = Some(stroke_to_usvg_stroke(stroke));
        }
        svg_path
    })
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
            let (x2, y2) = part2.main_point();
            let x = layout.eval(x, parent_id);
            let y = layout.eval(y, parent_id);
            let x2 = layout.eval(x2, parent_id);
            let y2 = layout.eval(y2, parent_id);
            (x, y, x - x2, y - y2)
        }
        PathPart::Quad { .. } => {
            todo!()
        }
        PathPart::Cubic { .. } => {
            todo!()
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
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
    is_end_arrow: bool,
) -> Option<usvg::Path> {
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

    let mut builder = PathBuilder::new();
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

    builder.finish().map(|p| {
        let mut svg_path = usvg::Path::new(Rc::new(p));
        if let Some(width) = arrow.stroke_width {
            svg_path.stroke = Some(usvg::Stroke {
                paint: usvg::Paint::Color((color).into()),
                width: NonZeroPositiveF32::new(width).unwrap(),
                ..Default::default()
            });
        } else {
            svg_path.fill = Some(Fill {
                paint: usvg::Paint::Color(color.into()),
                opacity: color.opacity(),
                rule: Default::default(),
            });
        }
        svg_path
    })
}
