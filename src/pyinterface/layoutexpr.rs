use crate::model::{LayoutExpr, NodeId};
use pyo3::exceptions::PyValueError;
use pyo3::pybacked::PyBackedStr;
use pyo3::{FromPyObject, PyAny, PyResult};
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct PyLayoutExpr(LayoutExpr);

impl From<PyLayoutExpr> for LayoutExpr {
    fn from(value: PyLayoutExpr) -> Self {
        value.0
    }
}

impl<'py> FromPyObject<'py> for PyLayoutExpr {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        extract_layout_expr(ob.getattr("_expr")?).map(PyLayoutExpr)
    }
}

fn extract_layout_expr(obj: &PyAny) -> PyResult<LayoutExpr> {
    if let Ok(value) = obj.extract() {
        return Ok(LayoutExpr::ConstValue { value });
    }
    let name: PyBackedStr = obj.get_item(0)?.extract()?;
    match name.deref() {
        "x" => Ok(LayoutExpr::X {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
        }),
        "y" => Ok(LayoutExpr::Y {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
        }),
        "width" => Ok(LayoutExpr::Width {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            fraction: obj.get_item(2)?.extract()?,
        }),
        "height" => Ok(LayoutExpr::Height {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            fraction: obj.get_item(2)?.extract()?,
        }),
        "sum" => {
            let len = obj.len()?;
            Ok(LayoutExpr::Sum {
                expressions: (1..len)
                    .map(|idx| extract_layout_expr(obj.get_item(idx)?))
                    .collect::<PyResult<Vec<LayoutExpr>>>()?,
            })
        }
        "line_x" => Ok(LayoutExpr::LineX {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            line_idx: obj.get_item(2)?.extract()?,
        }),
        "line_y" => Ok(LayoutExpr::LineY {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            line_idx: obj.get_item(2)?.extract()?,
        }),
        "line_width" => Ok(LayoutExpr::LineWidth {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            line_idx: obj.get_item(2)?.extract()?,
            fraction: obj.get_item(3)?.extract()?,
        }),
        "line_height" => Ok(LayoutExpr::LineHeight {
            node_id: NodeId::new(obj.get_item(1)?.extract()?),
            line_idx: obj.get_item(2)?.extract()?,
            fraction: obj.get_item(3)?.extract()?,
        }),
        "anchor_x" => Ok(LayoutExpr::InTextAnchorX {
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
        }),
        _ => Err(PyValueError::new_err("Invalid expression")),
    }
}
