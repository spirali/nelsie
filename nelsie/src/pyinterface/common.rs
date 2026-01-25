use pyo3::{Borrowed, FromPyObject, PyAny, PyErr, PyResult};
use renderer::Color;
use std::str::FromStr;

pub(crate) struct PyColor(Color);

impl<'py> FromPyObject<'_, 'py> for PyColor {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let s: &str = obj.extract()?;
        Ok(PyColor(Color::from_str(s).map_err(crate::Error::from)?))
    }
}

impl From<PyColor> for Color {
    fn from(value: PyColor) -> Self {
        value.0
    }
}
