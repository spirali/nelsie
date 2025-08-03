use pyo3::exceptions::PyValueError;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyAnyMethods;
use pyo3::{intern, Bound, FromPyObject, PyAny, PyResult, Python};
use renderer::{LayoutExpr, NodeId};

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
        /*"width" => Ok(LayoutExpr::Width {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            fraction: obj.get_item(2)?.extract()?,
        }),
        "height" => Ok(LayoutExpr::Height {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            fraction: obj.get_item(2)?.extract()?,
        }),*/
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
        /*"anchor_x" => Ok(LayoutExpr::InTextAnchorX {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            anchor_id: obj.get_item(2)?.extract()?,
        }),
        "anchor_y" => Ok(LayoutExpr::InTextAnchorY {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            anchor_id: obj.get_item(2)?.extract()?,
        }),
        "anchor_width" => Ok(LayoutExpr::InTextAnchorWidth {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            anchor_id: obj.get_item(2)?.extract()?,
            fraction: obj.get_item(3)?.extract()?,
        }),
        "anchor_height" => Ok(LayoutExpr::InTextAnchorHeight {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            anchor_id: obj.get_item(2)?.extract()?,
            fraction: obj.get_item(3)?.extract()?,
        }),*/
        _ => Err(PyValueError::new_err("Invalid expression")),
    }
}
