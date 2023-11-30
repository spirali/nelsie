use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use pyo3::{FromPyObject, PyResult};
use pyo3::exceptions::PyValueError;
use crate::model::StepValue;

#[derive(Debug, FromPyObject)]
pub(crate) struct InSteps<T> {
    pub in_step_values: HashMap<u32, T>,
    pub n_steps: u32,
}

#[derive(Debug, FromPyObject)]
pub(crate) enum ValueOrInSteps<T> {
    Value(T),
    InSteps(InSteps<T>)
}

impl<T> ValueOrInSteps<T> {
    pub fn parse<S, F: Fn(T) -> crate::Result<S>>(self, parser: F) -> PyResult<StepValue<S>> where S: Debug {
        match self {
            ValueOrInSteps::Value(v) => Ok(StepValue::Const(parser(v).map_err(|e| PyValueError::new_err(e.to_string()))?)),
            ValueOrInSteps::InSteps(_) => todo!()
        }
    }
}