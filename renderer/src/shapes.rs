use crate::render::canvas::Canvas;
use crate::render::draw::{DrawPath, DrawRect};
use crate::render::layout::ComputedLayout;
use crate::types::LayoutExpr;
use crate::{Color, NodeId, Rectangle};

#[derive(Clone, Debug, PartialEq)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
    pub dash_array: Option<Vec<f32>>,
    pub dash_offset: f32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FillAndStroke {
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
pub struct ShapeRect {
    pub x1: LayoutExpr,
    pub y1: LayoutExpr,
    pub x2: LayoutExpr,
    pub y2: LayoutExpr,
    pub z_level: i32,
    pub fill_and_stroke: FillAndStroke,
}

impl ShapeRect {
    pub fn new(
        x1: LayoutExpr,
        y1: LayoutExpr,
        x2: LayoutExpr,
        y2: LayoutExpr,
        z_level: i32,
        fill_and_stroke: FillAndStroke,
    ) -> Self {
        ShapeRect {
            x1,
            y1,
            x2,
            y2,
            z_level,
            fill_and_stroke,
        }
    }

    pub(crate) fn eval(&self, layout: &ComputedLayout, parent_id: NodeId) -> DrawRect {
        let x1 = layout.eval(&self.x1, parent_id);
        let y1 = layout.eval(&self.y1, parent_id);
        let x2 = layout.eval(&self.x2, parent_id);
        let y2 = layout.eval(&self.y2, parent_id);
        DrawRect {
            rectangle: Rectangle {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
            },
            fill_and_stroke: self.fill_and_stroke.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Shape {
    Rect(ShapeRect),
    Oval(ShapeRect),
    Path(Path),
}

#[derive(Debug)]
pub struct Arrow {
    pub size: f32,
    pub angle: f32,
    pub color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub inner_point: Option<f32>,
}

#[derive(Debug)]
pub enum PathPart {
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
pub struct Path {
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
