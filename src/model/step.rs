use crate::common::error::NelsieError;
use smallvec::{smallvec, SmallVec};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub(crate) type StepIndex = u32;

pub(crate) type StepSet = BTreeSet<Step>;

#[derive(Eq, PartialEq, Clone, Default)]
pub(crate) struct Step {
    indices: SmallVec<[StepIndex; 2]>,
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.indices.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl Debug for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.indices.iter().zip(&other.indices) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                x => return x,
            }
        }
        self.indices.len().cmp(&other.indices.len())
    }
}

impl FromStr for Step {
    type Err = NelsieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('.')
            .map(|v| {
                v.parse()
                    .map_err(|_| NelsieError::generic_err("Invalid step definition"))
            })
            .collect::<crate::Result<SmallVec<[StepIndex; 2]>>>()
            .map(|v| Step { indices: v })
    }
}

impl Step {
    pub fn from_int(index: StepIndex) -> Step {
        Step {
            indices: smallvec![index],
        }
    }

    pub fn from_vec(indices: Vec<StepIndex>) -> Step {
        assert!(!indices.is_empty());
        Step {
            indices: indices.into(),
        }
    }

    pub fn from_slice(indices: &[StepIndex]) -> Step {
        assert!(!indices.is_empty());
        Step {
            indices: indices.into(),
        }
    }

    pub fn share_prefix(&self, other: &Step) -> bool {
        for (a, b) in self.indices.iter().zip(&other.indices) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                _ => return false,
            }
        }
        true
    }

    pub fn indices(&self) -> &[StepIndex] {
        &self.indices
    }

    pub fn next(&self) -> Step {
        let mut indices = self.indices.clone();
        *indices.last_mut().unwrap() += 1;
        Step { indices }
    }

    pub fn first_substep(&self) -> Step {
        let mut indices = self.indices.clone();
        indices.push(0);
        Step { indices }
    }

    pub fn subtract_first_index(&self, index: StepIndex) -> Step {
        assert!(self.indices[0] >= index);
        let mut indices = self.indices.clone();
        indices[0] -= index;
        Step { indices }
    }

    pub fn add_first_index(&self, index: StepIndex) -> Step {
        let mut indices = self.indices.clone();
        indices[0] += index;
        Step { indices }
    }
}

#[cfg(test)]
mod test {
    use crate::model::Step;

    #[test]
    fn test_step_to_string() {
        assert_eq!(Step::from_int(15).to_string(), "15");
        assert_eq!(Step::from_slice(&[1, 5, 2]).to_string(), "1.5.2");
    }
}
