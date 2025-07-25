mod check;
mod extract;
mod rendering;
mod resources;

use crate::pyinterface::resources::Resources;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyInt};
use pyo3::PyTypeInfo;

/// A Python module implemented in Rust.
#[pymodule]
fn nelsie(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Resources>()?;
    m.add_function(wrap_pyfunction!(check::check_color, m)?)?;
    m.add_function(wrap_pyfunction!(rendering::render, m)?)?;
    Ok(())
}

impl From<crate::Error> for PyErr {
    fn from(err: crate::Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
