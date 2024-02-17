use itertools::Itertools;
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

    fn load_fonts_dir(&mut self, path: &str) -> PyResult<()> {
        self.resources.load_fonts_dir(path);
        Ok(())
    }

    fn syntaxes(&self) -> PyResult<Vec<(String, Vec<String>)>> {
        Ok(self
            .resources
            .syntax_set
            .syntaxes()
            .iter()
            .map(|s| (s.name.clone(), s.file_extensions.clone()))
            .collect_vec())
    }

    fn themes(&self) -> PyResult<Vec<String>> {
        Ok(self
            .resources
            .theme_set
            .themes
            .keys()
            .cloned()
            .collect_vec())
    }
}
