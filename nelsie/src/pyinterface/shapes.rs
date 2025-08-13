use crate::parsers::length::parse_string_length;
use crate::pyinterface::common::PyColor;
use crate::pyinterface::layoutexpr::extract_layout_expr;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use renderer::{FillAndStroke, LayoutExpr, Length, Shape, ShapeRect, Stroke};
use std::marker::PhantomData;

pub(crate) trait Dimension {
    fn parent_pos(shift: f32) -> LayoutExpr;
    fn parent_size(fraction: f32) -> LayoutExpr;
}

pub(crate) struct DimX;
pub(crate) struct DimY;

impl Dimension for DimX {
    fn parent_pos(shift: f32) -> LayoutExpr {
        LayoutExpr::ParentX { shift }
    }

    fn parent_size(fraction: f32) -> LayoutExpr {
        LayoutExpr::ParentWidth { fraction }
    }
}

impl Dimension for DimY {
    fn parent_pos(shift: f32) -> LayoutExpr {
        LayoutExpr::ParentY { shift }
    }

    fn parent_size(fraction: f32) -> LayoutExpr {
        LayoutExpr::ParentHeight { fraction }
    }
}

pub(crate) struct PyPosition<D: Dimension> {
    pub(crate) expr: LayoutExpr,
    _dim: PhantomData<D>,
}

impl<'py, D: Dimension> FromPyObject<'py> for PyPosition<D> {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyPosition {
            expr: if let Ok(value) = obj.extract::<f32>() {
                D::parent_pos(value)
            } else if let Ok(value) = obj.extract::<&str>() {
                D::parent_pos(0.0).add(match parse_string_length(value)? {
                    Length::Points { value } => D::parent_pos(value),
                    Length::Fraction { value } => D::parent_size(0.0).add(D::parent_size(value)),
                })
            } else {
                extract_layout_expr(obj)?
            },
            _dim: Default::default(),
        })
    }
}

#[derive(FromPyObject)]
pub(crate) struct PyStroke {
    color: PyColor,
    width: f32,
    dash_array: Option<Vec<f32>>,
    dash_offset: f32,
}

impl From<PyStroke> for Stroke {
    fn from(value: PyStroke) -> Self {
        Stroke {
            color: value.color.into(),
            width: value.width,
            dash_array: value.dash_array,
            dash_offset: value.dash_offset,
        }
    }
}

#[derive(FromPyObject)]
pub(crate) struct PyRect {
    shape: u32,
    x1: PyPosition<DimX>,
    y1: PyPosition<DimX>,
    x2: PyPosition<DimX>,
    y2: PyPosition<DimX>,
    z_level: i32,
    stroke: Option<PyStroke>,
    fill_color: Option<PyColor>,
}

impl PyRect {
    pub fn into_shape(self) -> Shape {
        let rect = ShapeRect {
            x1: self.x1.expr,
            y1: self.y1.expr,
            x2: self.x2.expr,
            y2: self.y2.expr,
            z_level: self.z_level,
            fill_and_stroke: FillAndStroke {
                fill_color: self.fill_color.map(|x| x.into()),
                stroke: self.stroke.map(|x| x.into()),
            },
        };
        if self.shape == 1 {
            Shape::Oval(rect)
        } else {
            Shape::Rect(rect)
        }
    }
}
