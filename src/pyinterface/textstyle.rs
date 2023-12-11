use crate::common::error::NelsieError;
use crate::model::NodeContent::Text;
use crate::model::{Color, PartialTextStyle, Resources};
use crate::parsers::parse_color;
use crate::pyinterface::insteps::ValueOrInSteps;
use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyAny, PyObject, PyResult, Python, ToPyObject};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Default)]
pub(crate) struct PyTextStyle(PartialTextStyle);

impl PyTextStyle {
    pub fn new(style: PartialTextStyle) -> Self {
        PyTextStyle(style)
    }
    pub fn to_partial_style(self, resources: &Resources) -> crate::Result<PartialTextStyle> {
        if let Some(font) = &self.0.font_family {
            resources.check_font(font)?;
        }
        Ok(self.0)
    }
}

impl<'py> FromPyObject<'py> for Color {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Color::from_str(ob.extract()?).map_err(|_| PyValueError::new_err("Invalid color"))
    }
}

impl<'py> FromPyObject<'py> for PyTextStyle {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(PyTextStyle(PartialTextStyle {
            font_family: ob
                .getattr("font_family")?
                .extract::<Option<String>>()?
                .map(Arc::new),
            color: ob.getattr("color")?.extract()?,
            size: ob.getattr("size")?.extract()?,
            line_spacing: ob.getattr("line_spacing")?.extract()?,
        }))
    }
}

impl ToPyObject for PyTextStyle {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let mut map: HashMap<String, PyObject> = HashMap::new();
        map.insert(
            "font_family".into(),
            self.0.font_family.as_deref().to_object(py),
        );
        map.insert(
            "color".into(),
            self.0.color.as_ref().map(|v| v.to_string()).to_object(py),
        );
        map.insert("size".into(), self.0.size.to_object(py));
        map.insert("line_spacing".into(), self.0.line_spacing.to_object(py));
        map.to_object(py)
    }
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyTextStyleOrName {
    Name(String),
    Style(ValueOrInSteps<PyTextStyle>),
}
