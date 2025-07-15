use crate::render::canvas::Canvas;
use crate::render::layout::ComputedLayout;
use crate::types::LayoutExpr;
use crate::{Color, NodeId};

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
pub(crate) struct Arrow {
    pub size: f32,
    pub angle: f32,
    pub color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub inner_point: Option<f32>,
}

#[derive(Debug)]
pub(crate) enum PathPart {
    Move {
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Line {
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Quad {
        x1: LayoutExpr,
        y1: LayoutExpr,
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Cubic {
        x1: LayoutExpr,
        y1: LayoutExpr,
        x2: LayoutExpr,
        y2: LayoutExpr,
        x: LayoutExpr,
        y: LayoutExpr,
    },
    Close,
}

impl PathPart {
    pub fn main_point(&self) -> Option<(&LayoutExpr, &LayoutExpr)> {
        match self {
            PathPart::Move { x, y }
            | PathPart::Line { x, y }
            | PathPart::Quad { x, y, .. }
            | PathPart::Cubic { x, y, .. } => Some((x, y)),
            PathPart::Close => None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Path {
    pub fill_and_stroke: FillAndStroke,
    pub parts: Vec<PathPart>,
    pub arrow_start: Option<Arrow>,
    pub arrow_end: Option<Arrow>,
}

impl Path {
    pub(crate) fn render_to_canvas(
        layout: &ComputedLayout,
        parent_id: NodeId,
        canvas: &mut Canvas,
    ) {
        todo!()
        // let mut builder = PathBuilder::new(path.fill_and_stroke.clone());
        // for (i, part) in path.parts.iter().enumerate() {
        //     let (sx, sy) = crate::render::paths::move_point_for_arrow(layout, parent_id, path, i).unwrap_or((0.0, 0.0));
        //     match part {
        //         PathPart::Move { x, y } => {
        //             builder.move_to(
        //                 layout.eval(x, parent_id) + sx,
        //                 layout.eval(y, parent_id) + sy,
        //             );
        //         }
        //         PathPart::Line { x, y } => {
        //             builder.line_to(
        //                 layout.eval(x, parent_id) + sx,
        //                 layout.eval(y, parent_id) + sy,
        //             );
        //         }
        //         PathPart::Quad { x1, y1, x, y } => builder.quad_to(
        //             layout.eval(x1, parent_id),
        //             layout.eval(y1, parent_id),
        //             layout.eval(x, parent_id),
        //             layout.eval(y, parent_id),
        //         ),
        //         PathPart::Cubic {
        //             x1,
        //             y1,
        //             x2,
        //             y2,
        //             x,
        //             y,
        //         } => builder.cubic_to(
        //             layout.eval(x1, parent_id),
        //             layout.eval(y1, parent_id),
        //             layout.eval(x2, parent_id),
        //             layout.eval(y2, parent_id),
        //             layout.eval(x, parent_id),
        //             layout.eval(y, parent_id),
        //         ),
        //         PathPart::Close => builder.close(),
        //         PathPart::Oval { .. } => { /* Ignoring Oval, it has to be first if it used */ }
        //     }
        // }
        // canvas.add_draw_item(DrawItem::Path(builder.build()));
        // crate::render::paths::create_arrow(canvas, path, layout, parent_id, true);
        // crate::render::paths::create_arrow(canvas, path, layout, parent_id, false);
        //}
    }
}
