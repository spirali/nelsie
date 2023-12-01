use crate::common::Step;
use crate::model::{Slide, StepValue};
use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyResult};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

#[derive(Debug, FromPyObject)]
pub(crate) struct InSteps<T> {
    pub in_step_values: BTreeMap<u32, T>,
    pub n_steps: u32,
}

#[derive(Debug, FromPyObject)]
pub(crate) enum ValueOrInSteps<T> {
    Value(T),
    InSteps(InSteps<T>),
}

impl<T: Debug> ValueOrInSteps<T> {
    pub fn parse<S, F: Fn(T) -> crate::Result<S>>(
        self,
        n_steps: &mut Step,
        parser: F,
    ) -> PyResult<StepValue<S>>
    where
        S: Debug,
    {
        match self {
            ValueOrInSteps::Value(v) => Ok(StepValue::new_const(
                parser(v).map_err(|e| PyValueError::new_err(e.to_string()))?,
            )),
            ValueOrInSteps::InSteps(_) => todo!(),
        }
    }

    pub fn to_step_value(self, n_steps: &mut Step) -> StepValue<T> {
        match self {
            ValueOrInSteps::Value(v) => StepValue::new_const(v),
            ValueOrInSteps::InSteps(v) => {
                *n_steps = (*n_steps).max(v.n_steps);
                StepValue::new_map(v.in_step_values)
            }
        }
    }
}
