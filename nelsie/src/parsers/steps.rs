use std::cmp::{Ordering, Reverse};
use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

pub(crate) type StepIndex = u32;

#[derive(Eq, PartialEq, Clone, Default)]
pub(crate) struct Step {
    indices: SmallVec<[StepIndex; 2]>,
}

impl Step {

    #[cfg(test)]
    pub fn from_int(index: StepIndex) -> Step {
        Step {
            indices: smallvec![index],
        }
    }

    #[cfg(test)]
    pub fn from_slice(indices: &[StepIndex]) -> Step {
        assert!(!indices.is_empty());
        Step {
            indices: indices.into(),
        }
    }

    pub fn indices(&self) -> &[StepIndex] {
        &self.indices
    }

    pub fn next(&mut self) {
        *self.indices.last_mut().unwrap() += 1;
    }

    pub fn first_substep(&mut self) {
        self.indices.push(0);
    }
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

pub fn parse_step(input: &str) -> crate::Result<(Step, bool, bool)> {
    let (input, exact) = input.strip_prefix('!').map(|s| (s, true)).unwrap_or((input, false));
    let (input, silent) = input.strip_suffix('?').map(|s| (s, true)).unwrap_or((input, false));
    let indices = input.split(".").map(|s| {
        let v: StepIndex = s.parse().map_err(|_| crate::Error::Parsing("Invalid step definition".to_string()))?;
        Ok(v)
    }).collect::<crate::Result<_>>()?;
    Ok((Step {
        indices,
    }, exact, silent))
}

pub fn parse_bool_steps(input: &str) -> crate::Result<(Vec<(Step, bool)>, Vec<Step>)>{
    let mut result = Vec::new();
    let mut named = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if let Some(part) = part.strip_suffix('+') {
            let (step, _, silent) = parse_step(part)?;
            if !silent {
                named.push(step.clone());
            }
            result.push((step, true));
        } else if let Some((part1, part2)) = part.split_once('-') {
            let part1 = part1.trim();
            let part2 = part2.trim();
            let (step1, _, silent1) = parse_step(part1)?;
            let (mut step2, exact, silent2) = parse_step(part2)?;
            if !silent1 {
                named.push(step1.clone());
            }
            if !silent2 {
                named.push(step2.clone());
            }
            result.push((step1, true));
            if exact {
                step2.first_substep();
            } else {
                step2.next();
            }
            result.push((step2, false));
        } else {
            let (step, exact, silent) = parse_step(part)?;
            if !silent {
                named.push(step.clone());
            }
            let mut end = step.clone();
            if exact {
                end.first_substep();
            } else {
                end.next();
            }
            result.push((step, true));
            result.push((end, false));
        }
    }
    result.sort_unstable_by(|(step1, onoff1), (step2, onoff2)| {
        let r = step1.cmp(step2);
        if r == Ordering::Equal {
            onoff1.cmp(&onoff2).reverse()
        } else {
            r
        }
    });
    result.dedup_by(|(step1, _), (step2, _) | step1 == step2);
    result.dedup_by_key(|a| a.1);
    named.sort_unstable();
    Ok((result, named))
}
