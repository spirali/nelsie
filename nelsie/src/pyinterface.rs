use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyInt};
use pyo3::PyTypeInfo;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn check_color<'py>(obj: &Bound<'py, PyModule>) -> PyResult<()> {
    if let Ok(s) = obj.extract::<&str>() {
        if renderer::Color::from_str(s).is_ok() {
            return Ok(());
        }
    }
    Err(PyException::new_err(format!(
        "Invalid color: '{}'",
        obj.to_string()
    )))
}

/// A Python module implemented in Rust.
#[pymodule]
fn nelsie(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_color, m)?)?;
    Ok(())
}
