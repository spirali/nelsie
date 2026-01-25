use pyo3::exceptions::PyException;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, PyAny, PyResult, pyfunction};
use std::str::FromStr;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub(crate) fn check_color<'py>(obj: &Bound<'py, PyAny>) -> PyResult<()> {
    if let Ok(s) = obj.extract::<&str>()
        && renderer::Color::from_str(s).is_ok()
    {
        return Ok(());
    }
    Err(PyException::new_err(format!("Invalid color: '{}'", obj)))
}
