use crate::model::{Step, StepSet, StepValue};

use pyo3::exceptions::PyException;
use pyo3::types::PyTuple;
use pyo3::{FromPyObject, PyAny, PyObject, PyResult, Python, ToPyObject};
use std::collections::BTreeMap;
use std::default::Default;
use std::fmt::Debug;

impl<'py> FromPyObject<'py> for Step {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<u32>() {
            return Ok(Step::from_int(v));
        }
        if let Ok(v) = ob.extract::<Vec<u32>>() {
            if v.is_empty() {
                return Err(PyException::new_err("Step cannot be an empty sequence"));
            }
            return Ok(Step::from_vec(v));
        }
        Err(PyException::new_err("Invalid step definition"))
    }
}

impl ToPyObject for Step {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyTuple::new_bound(py, self.indices()).into()
    }
}

#[derive(Debug)]
pub(crate) struct InSteps<T> {
    pub in_step_values: BTreeMap<Step, T>,
}

impl<'py, T: FromPyObject<'py>> FromPyObject<'py> for InSteps<T> {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(InSteps {
            in_step_values: ob.getattr("in_step_values")?.extract()?,
        })
    }
}

impl<T: Debug> InSteps<T> {
    pub fn parse<S, E, F: FnMut(T) -> Result<S, E>>(
        self,
        steps: &mut StepSet,
        mut parser: F,
    ) -> Result<StepValue<S>, E>
    where
        S: Debug + Default,
    {
        for step in self.in_step_values.keys() {
            steps.insert(step.clone());
        }
        Ok(StepValue::new_map(
            self.in_step_values
                .into_iter()
                .map(|(k, v)| parser(v).map(|v| (k, v)))
                .collect::<Result<BTreeMap<Step, S>, E>>()?,
        ))
    }

    pub fn into_step_value(self, steps: &mut StepSet) -> StepValue<T>
    where
        T: Default,
    {
        for step in self.in_step_values.keys() {
            steps.insert(step.clone());
        }
        StepValue::new_map(self.in_step_values)
    }
}

#[derive(Debug)]
pub(crate) enum ValueOrInSteps<T> {
    Value(T),
    InSteps(InSteps<T>),
}

impl<'py, T: FromPyObject<'py>> FromPyObject<'py> for ValueOrInSteps<T> {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(if ob.hasattr("in_step_values")? {
            ValueOrInSteps::InSteps(ob.extract()?)
        } else {
            ValueOrInSteps::Value(ob.extract()?)
        })
    }
}

impl<T: Debug> ValueOrInSteps<T> {
    pub fn parse<S, E, F: FnMut(T) -> Result<S, E>>(
        self,
        steps: &mut StepSet,
        mut parser: F,
    ) -> Result<StepValue<S>, E>
    where
        S: Debug + Default,
    {
        match self {
            ValueOrInSteps::Value(v) => Ok(StepValue::new_const(parser(v)?)),
            ValueOrInSteps::InSteps(in_steps) => in_steps.parse(steps, parser),
        }
    }

    pub fn parse_ignore_n_steps<S: Debug + Default, E, F: FnMut(T) -> Result<S, E>>(
        self,
        parser: F,
    ) -> Result<StepValue<S>, E> {
        let mut discard = Default::default();
        self.parse(&mut discard, parser)
    }

    pub fn into_step_value(self, steps: &mut StepSet) -> StepValue<T>
    where
        T: Default,
    {
        match self {
            ValueOrInSteps::Value(v) => StepValue::new_const(v),
            ValueOrInSteps::InSteps(v) => v.into_step_value(steps),
        }
    }
}
