use itertools::Itertools;
use pyo3::{pyclass, pymethods, PyResult};
use std::path::Path;

#[pyclass]
pub(crate) struct Resources {
    pub resources: renderer::Resources,
}

#[pymethods]
impl Resources {
    #[new]
    fn new(
        system_fonts: bool,
        system_fonts_for_svg: bool,
        default_code_syntaxes: bool,
        default_code_themes: bool,
    ) -> PyResult<Self> {
        Ok(Resources {
            resources: renderer::Resources::new(
                system_fonts,
                system_fonts_for_svg,
                default_code_syntaxes,
                default_code_themes,
            ),
        })
    }

    fn set_generic_family(&mut self, family_name: &str, font_name: &str) -> PyResult<()> {
        self.resources
            .set_generic_family(family_name, font_name)
            .map_err(crate::Error::from)?;
        Ok(())
    }

    fn load_code_syntax_dir(&mut self, path: &str) -> PyResult<()> {
        self.resources
            .load_code_syntax_dir(Path::new(path))
            .map_err(crate::Error::from)?;
        Ok(())
    }

    fn load_code_theme_dir(&mut self, path: &str) -> PyResult<()> {
        self.resources
            .load_code_theme_dir(Path::new(path))
            .map_err(crate::Error::from)?;
        Ok(())
    }

    fn load_fonts_dir(&mut self, path: &str) -> PyResult<()> {
        self.resources
            .load_fonts_dir(Path::new(path))
            .map_err(crate::Error::from)?;
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
