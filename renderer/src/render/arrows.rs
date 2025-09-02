use crate::render::draw::{DrawPath, DrawPathPart, PathBuilder};
use crate::shapes::{FillAndStroke, Stroke};
use crate::{Arrow, Color};

fn arrow_direction(
    part1: &DrawPathPart,
    part2: Option<&DrawPathPart>,
) -> Option<(f32, f32, f32, f32)> {
    let (x, y, dx, dy) = match part1 {
        DrawPathPart::Move { x, y } | DrawPathPart::Line { x, y } => {
            let part2 = part2?;
            let (x2, y2) = part2.main_point()?;
            (*x, *y, x - x2, y - y2)
        }
        DrawPathPart::Quad {
            x1: x2,
            y1: y2,
            x,
            y,
        }
        | DrawPathPart::Cubic { x2, y2, x, y, .. } => (*x, *y, x - x2, y - y2),
        DrawPathPart::Close => {
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
    arrow: &Arrow,
    part1: &mut DrawPathPart,
    part2: Option<&DrawPathPart>,
    default_color: Option<Color>,
) -> Option<DrawPath> {
    let color = arrow.color.or(default_color)?;
    let (x, y, dx, dy) = arrow_direction(part1, part2)?;
    let angle = arrow.angle * std::f32::consts::PI / 180.0;
    let a = dx.atan2(dy) + std::f32::consts::PI;
    let x1 = x + arrow.size * (a - angle).sin();
    let y1 = y + arrow.size * (a - angle).cos();
    let x2 = x + arrow.size * (a + angle).sin();
    let y2 = y + arrow.size * (a + angle).cos();

    let (p_x, p_y) = part1.main_point_mut()?;
    let len = if let Some(w) = arrow.stroke_width {
        w / 2.0
    } else {
        arrow.size * arrow.inner_point.unwrap_or(1.0) / 2.0
    };
    *p_x += -dx * len;
    *p_y += -dy * len;

    let fill_and_stroke = if let Some(width) = arrow.stroke_width {
        FillAndStroke::new_stroke(Stroke {
            color,
            width,
            dash_array: None,
            dash_offset: 0.0,
        })
    } else {
        FillAndStroke::new_fill(color)
    };
    let mut builder = PathBuilder::new(fill_and_stroke);
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
    Some(builder.build())
}
