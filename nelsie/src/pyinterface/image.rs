use pyo3::exceptions::PyValueError;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) type SharedDataMap = HashMap<usize, SharedData>;

pub(crate) enum SharedData {
    Bytes(Arc<Vec<u8>>),
    Str(Arc<String>),
}

#[derive(FromPyObject)]
pub(crate) struct PyPathImage {
    pub path: String,
}

#[derive(FromPyObject)]
pub(crate) struct PyMemImage {
    pub data_id: usize,
    pub format: PyImageFormat,
}

pub(crate) enum PyImageFormat {
    Png,
    Jpeg,
    Ora,
    Svg,
}

impl<'py> FromPyObject<'py> for PyImageFormat {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = ob.extract()?;
        Ok(match s {
            "png" => Self::Png,
            "jpeg" => Self::Jpeg,
            "ora" => Self::Ora,
            "svg" => Self::Svg,
            _ => return Err(PyValueError::new_err("Invalid file format")),
        })
    }
}
