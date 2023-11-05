use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Debug;

pub type Step = u32;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum StepValue<T: Debug> {
    Const(T),
}

impl<T: Debug + DeserializeOwned> StepValue<T> {
    pub fn new_const(value: T) -> Self {
        StepValue::Const(value)
    }

    pub fn get(&self, step: Step) -> &T {
        match self {
            StepValue::Const(v) => v,
        }
        /*match self.indices.get(step as usize) {
            Some(Some(idx)) => Some(&self.values[*idx as usize]),
            Some(None) => None,
            None => self.indices.last().unwrap().map(|idx| &self.values[idx as usize])
        }*/
    }
}
