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

    pub fn new_single_value(step: Step, value: T) -> Self {
        if step == Step::from_int(1) {
            StepValue::Const(value)
        } else {
            let mut map = BTreeMap::new();
            map.insert(Step::from_int(1), T::default());
            map.insert(step, value);
            StepValue::Steps(map)
        }
    }
}

impl<T: Debug> StepValue<T> {
    pub fn new_const(value: T) -> Self {
        StepValue::Const(value)
    }

    pub fn get_const(self) -> Option<T> {
        match self {
            StepValue::Const(v) => Some(v),
            StepValue::Steps(_) => None,
        }
    }

    pub fn at_step(&self, step: &Step) -> &T {
        match self {
            StepValue::Const(v) => v,
            StepValue::Steps(steps) => steps
                .range((Unbounded, Included(step)))
                .next_back()
                .map(|(_, v)| v)
                .unwrap_or_else(|| steps.first_key_value().unwrap().1),
        }
    }

    pub fn steps(&self) -> impl Iterator<Item = &Step> {
        match self {
            StepValue::Const(_) => itertools::Either::Left(std::iter::empty()),
            StepValue::Steps(v) => itertools::Either::Right(v.keys()),
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

    pub fn try_map_ref<E, S: Debug, F: FnMut(&T) -> Result<S, E>>(
        &self,
        mut f: F,
    ) -> Result<StepValue<S>, E> {
        match self {
            StepValue::Const(v) => f(v).map(StepValue::Const),
            StepValue::Steps(v) => v
                .iter()
                .map(|(k, v)| Ok((k.clone(), f(v)?)))
                .collect::<Result<_, E>>()
                .map(StepValue::Steps),
        }
    }

    // pub fn map<S: Debug, F: FnMut(T) -> S>(self, mut f: F) -> StepValue<S> {
    //     match self {
    //         StepValue::Const(v) => StepValue::Const(f(v)),
    //         StepValue::Steps(v) => {
    //             StepValue::Steps(v.into_iter().map(|(k, v)| (k, f(v))).collect())
    //         }
    //     }
    // }

    // pub fn map_ref<S: Debug, F: FnMut(&T) -> S>(&self, mut f: F) -> StepValue<S> {
    //     match self {
    //         StepValue::Const(v) => StepValue::Const(f(v)),
    //         StepValue::Steps(v) => {
    //             StepValue::Steps(v.iter().map(|(k, v)| (k.clone(), f(v))).collect())
    //         }
    //     }
    // }

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
