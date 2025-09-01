use pyo3::exceptions::PyValueError;
use pyo3::types::PyAnyMethods;
use pyo3::{intern, Bound, PyAny, PyResult};
use renderer::{InlineId, LayoutExpr, NodeId};

// #[derive(Debug)]
// pub(crate) struct PyLayoutExpr(LayoutExpr);
//
// impl From<PyLayoutExpr> for LayoutExpr {
//     fn from(value: PyLayoutExpr) -> Self {
//         value.0
//     }
// }
//
// impl<'py> FromPyObject<'py> for PyLayoutExpr {
//     fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//         Ok(crate::pyinterface::extract::PyLengthOrExpr(if let Ok(value) = obj.extract::<f32>() {
//             LengthOrExpr::points(value)
//         } else if let Ok(value) = obj.extract::<&str>() {
//         extract_layout_expr(ob.getattr("_expr")?).map(PyLayoutExpr)
//     }
// }

pub(crate) fn extract_layout_expr(obj: &Bound<PyAny>) -> PyResult<LayoutExpr> {
    if let Ok(value) = obj.extract() {
        return Ok(LayoutExpr::const_value(value));
    }
    let py = obj.py();
    let arg0 = intern!(py, "_arg0");
    let v0 = obj.getattr(arg0)?;
    let arg1 = intern!(py, "_arg1");
    let v1 = obj.getattr(arg1)?;
    let arg2 = intern!(py, "_arg2");
    let op = obj.getattr(intern!(py, "_op"))?;
    let name: &str = op.extract()?;
    match name {
        "+" | "-" | "*" => {
            let expr_a = extract_layout_expr(&v0)?;
            let expr_b = extract_layout_expr(&v1)?;
            Ok(match name {
                "+" => LayoutExpr::add(expr_a, expr_b),
                "-" => LayoutExpr::sub(expr_a, expr_b),
                "*" => LayoutExpr::mul(expr_a, expr_b),
                _ => unreachable!(),
            })
        }
        "x" => Ok(LayoutExpr::X {
            node_id: NodeId::new(v0.extract()?),
        }),
        "y" => Ok(LayoutExpr::Y {
            node_id: NodeId::new(v0.extract()?),
        }),
        "width" => Ok(LayoutExpr::Width {
            node_id: NodeId::new(v0.extract()?),
            fraction: v1.extract()?,
        }),
        "height" => Ok(LayoutExpr::Height {
            node_id: NodeId::new(v0.extract()?),
            fraction: v1.extract()?,
        }),
        "line_x" => Ok(LayoutExpr::LineX {
            node_id: NodeId::new(v0.extract()?),
            line_idx: v1.extract()?,
        }),
        "line_y" => Ok(LayoutExpr::LineY {
            node_id: NodeId::new(v0.extract()?),
            line_idx: v1.extract()?,
        }),
        "line_width" => Ok(LayoutExpr::LineWidth {
            node_id: NodeId::new(v0.extract()?),
            line_idx: v1.extract()?,
            fraction: obj.getattr(arg2)?.extract()?,
        }),
        "line_height" => Ok(LayoutExpr::LineHeight {
            node_id: NodeId::new(v0.extract()?),
            line_idx: v1.extract()?,
            fraction: obj.getattr(arg2)?.extract()?,
        }),
        "inline_x" => Ok(LayoutExpr::InlineX {
            node_id: NodeId::new(v0.extract()?),
            inline_id: InlineId::new(v1.extract()?),
        }),
        "inline_y" => Ok(LayoutExpr::InlineY {
            node_id: NodeId::new(v0.extract()?),
            inline_id: InlineId::new(v1.extract()?),
        }),
        "inline_width" => Ok(LayoutExpr::InlineWidth {
            node_id: NodeId::new(v0.extract()?),
            inline_id: InlineId::new(v1.extract()?),
            fraction: obj.getattr(arg2)?.extract()?,
        }),
        "inline_height" => Ok(LayoutExpr::InlineHeight {
            node_id: NodeId::new(v0.extract()?),
            inline_id: InlineId::new(v1.extract()?),
            fraction: obj.getattr(arg2)?.extract()?,
        }),
        "max" => Ok(LayoutExpr::max({
            let v: Vec<Bound<PyAny>> = v0.extract()?;
            v.into_iter()
                .map(|obj| extract_layout_expr(&obj))
                .collect::<PyResult<_>>()?
        })),
        _ => Err(PyValueError::new_err("Invalid expression")),
    }
}
