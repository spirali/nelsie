use crate::model::{LayoutExpr, NodeId};
use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyAny, PyResult};

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
    let name: &str = obj.get_item(0)?.extract()?;
    match name {
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
        _ => Err(PyValueError::new_err("Invalid expression")),
    }
}
