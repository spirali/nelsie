use pyo3::types::{PyDict, PyDictMethods, PyList};
use pyo3::{pyfunction, Bound, IntoPyObject, IntoPyObjectExt, PyAny, PyResult, Python};
use std::collections::BTreeMap;

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
    let result = PyDict::new(py);
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
