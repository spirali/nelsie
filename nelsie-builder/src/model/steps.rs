use crate::common::deutils::deserialize_int_key_map;
use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::collections::Bound::Included;
use std::fmt::{Debug, Display, Write};
use std::hash::Hash;
use std::ops::Bound::Unbounded;
use std::str::FromStr;

pub type Step = u32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
    #[serde(deserialize_with = "deserialize_int_key_map")]
    Steps(BTreeMap<Step, T>),
}

impl<T: Debug> StepValue<T> {
    pub fn from_btree(tree: BTreeMap<Step, T>) -> Self {
        StepValue::Steps(tree)
    }
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
                .range((Unbounded, Included(&step)))
                .next_back()
                .map(|(_, v)| v)
                .unwrap_or_else(|| {
                    dbg!(step);
                    dbg!(self);
                    panic!("Step not found")
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
