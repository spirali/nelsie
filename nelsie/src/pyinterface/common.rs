use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use renderer::Color;
use std::str::FromStr;

pub(crate) struct PyColor(Color);

impl<'py> FromPyObject<'py> for PyColor {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = obj.extract()?;
        Ok(PyColor(Color::from_str(s).map_err(crate::Error::from)?))
    }
}

impl From<PyColor> for Color {
    fn from(value: PyColor) -> Self {
        value.0
    }
}
