use std::collections::BTreeMap;
use std::collections::Bound::Included;
use std::fmt::{Debug, Display, Write};
use std::hash::Hash;
use std::ops::Bound::Unbounded;
use std::str::FromStr;
use crate::model::Length;

pub type Step = u32;

#[derive(Debug)]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
    Steps(BTreeMap<Step, T>),
}

impl<T: Debug> StepValue<T> {
    pub fn from_btree(tree: BTreeMap<Step, T>) -> Self {
        StepValue::Steps(tree)
    }
}

impl<T: Debug> StepValue<T> {
    pub fn new_const(value: T) -> Self {
        StepValue::Const(value)
    }

    pub fn at_step(&self, step: Step) -> &T {
        assert!(step > 0);
        match self {
            StepValue::Const(v) => v,
            StepValue::Steps(steps) => steps
                .range((Unbounded, Included(&step)))
                .next_back()
                .map(|(_, v)| v)
                .unwrap_or_else(|| {
                    panic!("Invalid step")
                }),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        match self {
            StepValue::Const(v) => itertools::Either::Left(std::iter::once(v)),
            StepValue::Steps(v) => itertools::Either::Right(v.values()),
        }
    }
}
