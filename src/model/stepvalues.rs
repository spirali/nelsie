use std::collections::BTreeMap;
use std::collections::Bound::Included;
use std::fmt::Debug;

use crate::model::step::Step;
use std::ops::Bound::Unbounded;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
    Steps(BTreeMap<Step, T>),
}

impl<T: Debug + Default> StepValue<T> {
    pub fn new_map(mut value: BTreeMap<Step, T>) -> Self {
        if let Some((k, _)) = value.first_key_value() {
            if k > &Step::from_int(1) {
                value.insert(Step::from_int(0), T::default());
            }
        }
        StepValue::Steps(value)
    }
}

impl<T: Debug> StepValue<T> {
    pub fn new_const(value: T) -> Self {
        StepValue::Const(value)
    }

    pub fn at_step(&self, step: &Step) -> &T {
        debug_assert!(step >= &Step::from_int(1));
        match self {
            StepValue::Const(v) => v,
            StepValue::Steps(steps) => steps
                .range((Unbounded, Included(step)))
                .next_back()
                .map(|(_, v)| v)
                .unwrap_or_else(|| panic!("Invalid step {}", step)),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        match self {
            StepValue::Const(v) => itertools::Either::Left(std::iter::once(v)),
            StepValue::Steps(v) => itertools::Either::Right(v.values()),
        }
    }

    #[cfg(test)]
    pub fn key_values(self) -> impl Iterator<Item = (Step, T)> {
        match self {
            StepValue::Const(_) => itertools::Either::Left(std::iter::empty()),
            StepValue::Steps(v) => itertools::Either::Right(v.into_iter()),
        }
    }

    pub fn map<S: Debug, F: FnMut(T) -> S>(self, mut f: F) -> StepValue<S> {
        match self {
            StepValue::Const(v) => StepValue::Const(f(v)),
            StepValue::Steps(v) => {
                StepValue::Steps(v.into_iter().map(|(k, v)| (k, f(v))).collect())
            }
        }
    }

    pub fn map_ref<S: Debug, F: FnMut(&T) -> S>(&self, mut f: F) -> StepValue<S> {
        match self {
            StepValue::Const(v) => StepValue::Const(f(v)),
            StepValue::Steps(v) => {
                StepValue::Steps(v.iter().map(|(k, v)| (k.clone(), f(v))).collect())
            }
        }
    }

    pub fn merge<S: Debug, R: Debug, F: FnMut(&T, &S) -> R>(
        &self,
        other: &StepValue<S>,
        mut f: F,
    ) -> StepValue<R> {
        match (self, other) {
            (StepValue::Const(v1), StepValue::Const(v2)) => StepValue::Const(f(v1, v2)),
            (StepValue::Steps(v1), StepValue::Const(v2)) => {
                StepValue::Steps(v1.iter().map(|(k, v)| (k.clone(), f(v, v2))).collect())
            }
            (StepValue::Const(v1), StepValue::Steps(v2)) => {
                StepValue::Steps(v2.iter().map(|(k, v)| (k.clone(), f(v1, v))).collect())
            }
            (StepValue::Steps(_v1), StepValue::Steps(_v2)) => {
                todo!()
            }
        }
    }
}
