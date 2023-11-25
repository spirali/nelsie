use crate::model::{Path, PathPart};
use crate::render::layout::ComputedLayout;
use resvg::tiny_skia::PathBuilder;
use std::rc::Rc;
use usvg::{NonZeroPositiveF32, NormalizedF32};

pub(crate) fn create_path(layout: &ComputedLayout, path: &Path) -> Option<usvg::Path> {
    let mut builder = PathBuilder::new();
    for part in &path.parts {
        match part {
            PathPart::Move { x, y } => {
                builder.move_to(layout.eval(x), layout.eval(y));
            }
            PathPart::Line { x, y } => {
                builder.line_to(layout.eval(x), layout.eval(y));
            }
            PathPart::Quad { x1, y1, x, y } => builder.quad_to(
                layout.eval(x1),
                layout.eval(y1),
                layout.eval(x),
                layout.eval(y),
            ),
            PathPart::Cubic {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => builder.cubic_to(
                layout.eval(x1),
                layout.eval(y1),
                layout.eval(x2),
                layout.eval(y2),
                layout.eval(x),
                layout.eval(y),
            ),
        }
    }
    builder.finish().map(|p| {
        let mut svg_path = usvg::Path::new(Rc::new(p));
        if let Some(stroke) = &path.stroke {
            svg_path.stroke = Some(usvg::Stroke {
                paint: usvg::Paint::Color((&stroke.color).into()),
                dasharray: stroke.dash_array.clone(),
                dashoffset: stroke.dash_offset,
                miterlimit: Default::default(),
                opacity: NormalizedF32::ONE,
                width: NonZeroPositiveF32::new(stroke.width).unwrap(),
                linecap: Default::default(),
                linejoin: Default::default(),
            });
        }
        svg_path
    })
}
