use crate::model::{NodeId, Path, PathPart};
use crate::render::layout::ComputedLayout;
use resvg::tiny_skia::PathBuilder;
use std::rc::Rc;
use usvg::{NonZeroPositiveF32, NormalizedF32};

pub(crate) fn create_path(
    layout: &ComputedLayout,
    parent_id: NodeId,
    path: &Path,
) -> Option<usvg::Path> {
    let mut builder = PathBuilder::new();
    for part in &path.parts {
        match part {
            PathPart::Move { x, y } => {
                builder.move_to(layout.eval(x, parent_id), layout.eval(y, parent_id));
            }
            PathPart::Line { x, y } => {
                builder.line_to(layout.eval(x, parent_id), layout.eval(y, parent_id));
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
            svg_path.stroke = Some(usvg::Stroke {
                paint: usvg::Paint::Color((&stroke.color).into()),
                dasharray: stroke.dash_array.clone(),
                dashoffset: stroke.dash_offset,
                miterlimit: Default::default(),
                opacity: stroke.color.opacity(),
                width: NonZeroPositiveF32::new(stroke.width).unwrap(),
                linecap: Default::default(),
                linejoin: Default::default(),
            });
        }
        svg_path
    })
}
