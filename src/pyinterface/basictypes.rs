use crate::model::LayoutExpr;
use crate::parsers::{StringOrFloat, StringOrFloatOrExpr};
use pyo3::FromPyObject;

#[derive(Debug, FromPyObject)]
pub(crate) enum PyStringOrFloat {
    Float(f32),
    String(String),
}

impl From<PyStringOrFloat> for StringOrFloat {
    fn from(value: PyStringOrFloat) -> Self {
        match value {
            PyStringOrFloat::Float(v) => StringOrFloat::Float(v),
            PyStringOrFloat::String(v) => StringOrFloat::String(v),
        }
    }
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyStringOrFloatOrExpr {
    Float(f32),
    String(String),
    //Expr(LayoutExpr),
}

impl From<PyStringOrFloatOrExpr> for StringOrFloatOrExpr {
    fn from(value: PyStringOrFloatOrExpr) -> Self {
        match value {
            PyStringOrFloatOrExpr::Float(v) => StringOrFloatOrExpr::Float(v),
            PyStringOrFloatOrExpr::String(v) => StringOrFloatOrExpr::String(v),
        }
    }
}
//
// impl From<&PyStringOrFloatOrExpr> for StringOrFloatOrExpr {
//     fn from(value: &PyStringOrFloatOrExpr) -> Self {
//         match value {
//             PyStringOrFloatOrExpr::Float(v) => StringOrFloatOrExpr::Float(*v),
//             PyStringOrFloatOrExpr::String(v) => StringOrFloatOrExpr::String(v.clone()),
//         }
//     }
// }
