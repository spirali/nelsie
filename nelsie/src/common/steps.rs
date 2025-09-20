use pyo3::exceptions::PyException;
use pyo3::types::{PyAnyMethods, PyTuple};
use pyo3::{Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr, PyResult, Python};
use smallvec::{smallvec, SmallVec};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

pub(crate) type StepIndex = u32;

#[derive(Eq, PartialEq, Clone, Default)]
pub(crate) struct Step {
    indices: SmallVec<[StepIndex; 2]>,
}

impl Step {
    pub fn new(indices: SmallVec<[StepIndex; 2]>) -> Self {
        Step { indices }
    }

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

impl<'py> IntoPyObject<'py> for &Step {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let indices = self.indices();
        Ok(if indices.len() == 1 {
            indices[0].into_pyobject(py)?.into_any()
        } else {
            PyTuple::new(py, indices)?.into_bound_py_any(py)?
        })
    }
}

impl<'py> FromPyObject<'py> for Step {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(v) = ob.extract::<u32>() {
            return Ok(Step::from_int(v));
        }
        if let Ok(v) = ob.extract::<Vec<u32>>() {
            if v.is_empty() {
                return Err(PyException::new_err("Step cannot be an empty sequence"));
            }
            return Ok(Step::from_vec(v));
        }
        Err(PyException::new_err("Invalid step definition"))
    }
}

pub(crate) fn bool_at_step(steps: &[(Step, bool)], step: &Step) -> bool {
    steps
        .iter()
        .filter(|(s, _)| s <= step)
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, v)| *v)
        .unwrap_or(false)
}
