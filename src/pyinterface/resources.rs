use pyo3::{pyclass, pymethods, PyResult};

#[pyclass]
pub(crate) struct Resources {
    pub resources: crate::model::Resources,
}

#[pymethods]
impl Resources {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Resources {
            resources: crate::model::Resources::new(),
        })
    }
}
