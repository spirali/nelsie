use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Debug;

pub type Step = u32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
    Steps(Vec<T>),
}

impl<T: Debug + DeserializeOwned> StepValue<T> {
    pub fn new_const(value: T) -> Self {
        StepValue::Const(value)
    }

    pub fn at_step(&self, step: Step) -> &T {
        assert!(step > 0);
        match self {
            StepValue::Const(v) => v,
            StepValue::Steps(steps) => steps
                .get((step - 1) as usize)
                .unwrap_or_else(|| steps.last().unwrap()),
        }
    }

    pub fn values(&self) -> impl Iterator<Item=&T> {
        match self {
            StepValue::Const(v) => itertools::Either::Left(std::iter::once(v)),
            StepValue::Steps(v) => itertools::Either::Right(v.iter()),
        }
    }
}
