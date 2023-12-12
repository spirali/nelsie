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

impl<T: Debug> InSteps<T> {
    pub fn parse<S, E, F: FnMut(T) -> Result<S, E>>(
        self,
        n_steps: &mut Step,
        mut parser: F,
    ) -> Result<StepValue<S>, E>
    where
        S: Debug + Default,
    {
        *n_steps = (*n_steps).max(self.n_steps);
        Ok(StepValue::new_map(
            self.in_step_values
                .into_iter()
                .map(|(k, v)| parser(v).map(|v| (k, v)))
                .collect::<Result<BTreeMap<Step, S>, E>>()?,
        ))
    }

    pub fn to_step_value(self, n_steps: &mut Step) -> StepValue<T>
    where
        T: Default,
    {
        *n_steps = (*n_steps).max(self.n_steps);
        StepValue::new_map(self.in_step_values)
    }
}

#[derive(Debug, FromPyObject)]
pub(crate) enum ValueOrInSteps<T> {
    Value(T),
    InSteps(InSteps<T>),
}

impl<T: Debug> ValueOrInSteps<T> {
    pub fn parse<S, E, F: FnMut(T) -> Result<S, E>>(
        self,
        n_steps: &mut Step,
        mut parser: F,
    ) -> Result<StepValue<S>, E>
    where
        S: Debug + Default,
    {
        match self {
            ValueOrInSteps::Value(v) => Ok(StepValue::new_const(parser(v)?)),
            ValueOrInSteps::InSteps(in_steps) => in_steps.parse(n_steps, parser),
        }
    }

    pub fn parse_ignore_n_steps<S: Debug + Default, E, F: FnMut(T) -> Result<S, E>>(
        self,
        parser: F,
    ) -> Result<StepValue<S>, E> {
        let mut discard = 1;
        self.parse(&mut discard, parser)
    }

    pub fn to_step_value(self, n_steps: &mut Step) -> StepValue<T>
    where
        T: Default,
    {
        match self {
            ValueOrInSteps::Value(v) => StepValue::new_const(v),
            ValueOrInSteps::InSteps(v) => v.to_step_value(n_steps),
        }
    }
}
