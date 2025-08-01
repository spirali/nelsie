use crate::parsers::steps::Step;
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyList, PyTuple};
use pyo3::{pyfunction, Borrowed, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr, PyResult, Python};
use std::collections::{BTreeMap, HashMap};
use pyo3::exceptions::PyException;

impl<'a, 'py> IntoPyObject<'py> for &'a Step {
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


// fn step_to_pyobj<'py>(py: Python<'py>, step: &Step) -> PyResult<Bound<'py, PyAny>> {
//     let indices = step.indices();
//     Ok(if indices.len() == 1 {
//         indices[0].into_bound_py_any(py)?
//     } else {
//         PyTuple::new(py, indices)?.into_bound_py_any(py)?
//     })
// }

#[pyfunction]
pub(crate) fn parse_bool_steps<'py>(
    py: Python<'py>,
    input: &str,
) -> PyResult<(Bound<'py, PyAny>, Bound<'py, PyAny>)> {
    let mut objs = BTreeMap::new();
    let (steps, named) = super::super::parsers::steps::parse_bool_steps(input)?;
    let mut result = PyDict::new(py);
    for (step, value) in steps {
        let s = step.into_pyobject(py)?.into_any();
        result.set_item(s.clone(), value.into_bound_py_any(py)?)?;
        objs.insert(step, s);
    }

    let named_list = PyList::new(
        py,
        named.into_iter().map(|step| {
            objs.remove(&step)
                .unwrap_or_else(|| step.into_pyobject(py).unwrap())
        }),
    );

    Ok((
        result.into_bound_py_any(py)?,
        named_list?.into_bound_py_any(py)?,
    ))
}
