use crate::model::{Color, PartialTextStyle, Resources, Stroke};

use crate::pyinterface::insteps::ValueOrInSteps;
use pyo3::exceptions::PyValueError;
use pyo3::{FromPyObject, PyAny, PyObject, PyResult, Python, ToPyObject};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use usvg_tree::FontStretch;

#[derive(Debug, Default)]
pub(crate) struct PyTextStyle(PartialTextStyle);

impl PyTextStyle {
    pub fn new(style: PartialTextStyle) -> Self {
        PyTextStyle(style)
    }
    pub fn into_partial_style(self, resources: &Resources) -> crate::Result<PartialTextStyle> {
        if let Some(font) = &self.0.font_family {
            resources.check_font(font)?;
        }
        Ok(self.0)
    }
}

impl<'py> FromPyObject<'py> for Color {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        Ok(Color::from_str(ob.extract()?)?)
    }
}

impl<'py> FromPyObject<'py> for PyTextStyle {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        let stretch_idx: Option<u32> = ob.getattr("stretch")?.extract()?;
        let stretch = stretch_idx
            .map(|s| match s {
                1 => Ok(FontStretch::UltraCondensed),
                2 => Ok(FontStretch::ExtraCondensed),
                3 => Ok(FontStretch::Condensed),
                4 => Ok(FontStretch::SemiCondensed),
                5 => Ok(FontStretch::Normal),
                6 => Ok(FontStretch::SemiExpanded),
                7 => Ok(FontStretch::Expanded),
                8 => Ok(FontStretch::ExtraExpanded),
                9 => Ok(FontStretch::UltraExpanded),
                _ => Err(PyValueError::new_err("Invalid font stretch")),
            })
            .transpose()?;
        let color = ob
            .getattr("color")?
            .extract::<Option<&str>>()?
            .map(|c| -> PyResult<_> {
                if c.trim() == "empty" {
                    Ok(None)
                } else {
                    Ok(Some(Color::from_str(c)?))
                }
            })
            .transpose()?;
        let stroke_attr = ob.getattr("stroke")?;
        let stroke = if let Ok(s) = stroke_attr.extract::<&str>() {
            if s == "empty" {
                Some(None)
            } else {
                return Err(PyValueError::new_err("Invalid stroke value"));
            }
        } else {
            stroke_attr
                .extract::<Option<Stroke>>()?
                .map(|x| Some(Arc::new(x)))
        };
        Ok(PyTextStyle(PartialTextStyle {
            font_family: ob
                .getattr("font_family")?
                .extract::<Option<String>>()?
                .map(Arc::new),
            stroke,
            color,
            size: ob.getattr("size")?.extract()?,
            line_spacing: ob.getattr("line_spacing")?.extract()?,
            italic: ob.getattr("italic")?.extract()?,
            stretch,
            weight: ob.getattr("weight")?.extract()?,
        }))
    }
}

fn stroke_to_py_map(stroke: &Stroke, py: Python<'_>) -> PyObject {
    let mut map = HashMap::new();
    map.insert("color".to_string(), stroke.color.to_string().to_object(py));
    map.insert("width".to_string(), stroke.width.to_object(py));
    map.insert("dash_array".to_string(), stroke.dash_array.to_object(py));
    map.insert("dash_offset".to_string(), stroke.dash_offset.to_object(py));
    map.to_object(py)
}

impl ToPyObject for PyTextStyle {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let stretch_idx = self.0.stretch.map(|s| match s {
            FontStretch::UltraCondensed => 1,
            FontStretch::ExtraCondensed => 2,
            FontStretch::Condensed => 3,
            FontStretch::SemiCondensed => 4,
            FontStretch::Normal => 5,
            FontStretch::SemiExpanded => 6,
            FontStretch::Expanded => 7,
            FontStretch::ExtraExpanded => 8,
            FontStretch::UltraExpanded => 9,
        });
        let mut map: HashMap<String, PyObject> = HashMap::new();
        map.insert(
            "font_family".into(),
            self.0.font_family.as_deref().to_object(py),
        );
        map.insert(
            "color".into(),
            self.0
                .color
                .as_ref()
                .map(|v| {
                    v.as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "empty".to_string())
                })
                .to_object(py),
        );
        map.insert(
            "stroke".into(),
            self.0
                .stroke
                .as_ref()
                .map(|v| {
                    v.as_ref()
                        .map(|s| stroke_to_py_map(s, py))
                        .unwrap_or_else(|| "empty".to_object(py))
                })
                .to_object(py),
        );
        map.insert("size".into(), self.0.size.to_object(py));
        map.insert("line_spacing".into(), self.0.line_spacing.to_object(py));
        map.insert("italic".into(), self.0.italic.to_object(py));
        map.insert("stretch".into(), stretch_idx.to_object(py));
        map.insert("weight".into(), self.0.weight.to_object(py));
        map.to_object(py)
    }
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyTextStyleOrName {
    Name(String),
    Style(ValueOrInSteps<PyTextStyle>),
}
