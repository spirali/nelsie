use crate::common::deutils::de_int_key;
use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::collections::Bound::Included;
use std::fmt::{Debug, Display, Write};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Bound::Unbounded;
use std::str::FromStr;

pub type Step = u32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
    #[serde(deserialize_with = "de_int_key")]
    Steps(BTreeMap<Step, T>),
}

impl<T: Debug> StepValue<T> {
    pub fn from_btree(tree: BTreeMap<Step, T>) -> Self {
        StepValue::Steps(tree)
    }
}

impl<T: Debug + DeserializeOwned + Default> StepValue<T> {
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
                .unwrap(),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        match self {
            StepValue::Const(v) => itertools::Either::Left(std::iter::once(v)),
            StepValue::Steps(v) => itertools::Either::Right(v.values()),
        }
    }
}
