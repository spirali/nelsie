use crate::model::{PartialTextStyle, Resources};

use crate::common::error::NelsieError;
use crate::common::Color;
use crate::pyinterface::insteps::ValueOrInSteps;
use pyo3::exceptions::PyValueError;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyAnyMethods, PyDict};
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult, Python};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use svg2pdf::usvg;
use svg2pdf::usvg::PositiveF32;
use usvg::FontStretch;

#[derive(Debug, Default)]
pub(crate) struct PyTextStyle {
    pub font_family: Option<String>,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub line_spacing: Option<f32>,
    pub italic: Option<bool>,
    pub stretch: Option<FontStretch>,
    pub weight: Option<u16>,
    pub underline: Option<bool>,
    pub line_through: Option<bool>,
}

pub fn to_positive_f32(value: f32) -> crate::Result<PositiveF32> {
    PositiveF32::new(value).ok_or_else(|| {
        NelsieError::generic_err(format!("Expected non-negative float, got {value}"))
    })
}

impl PyTextStyle {
    pub fn into_partial_style(self, resources: &mut Resources) -> crate::Result<PartialTextStyle> {
        let font = self
            .font_family
            .map(|name| resources.check_font(&name).map(Arc::new))
            .transpose()?;
        Ok(PartialTextStyle {
            font,
            color: self.color,
            size: self.size.map(to_positive_f32).transpose()?,
            line_spacing: self.line_spacing.map(to_positive_f32).transpose()?,
            italic: self.italic,
            stretch: self.stretch,
            weight: self.weight,
            underline: self.underline,
            line_through: self.line_through,
        })
    }
}

impl<'py> FromPyObject<'py> for Color {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: PyBackedStr = ob.extract()?;
        Ok(Color::from_str(s.deref())?)
    }
}

impl<'py> FromPyObject<'py> for PyTextStyle {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
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
        Ok(PyTextStyle {
            font_family: ob.getattr("font_family")?.extract::<Option<String>>()?,
            color: ob.getattr("color")?.extract()?,
            size: ob.getattr("size")?.extract()?,
            line_spacing: ob.getattr("line_spacing")?.extract()?,
            italic: ob.getattr("italic")?.extract()?,
            stretch,
            weight: ob.getattr("weight")?.extract()?,
            underline: ob.getattr("underline")?.extract()?,
            line_through: ob.getattr("line_through")?.extract()?,
        })
    }
}

pub fn partial_text_style_to_pyobject<'py>(
    style: &PartialTextStyle,
    py: Python<'py>,
) -> PyResult<Bound<'py, PyDict>> {
    let stretch_idx = style.stretch.map(|s| match s {
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
    let mut map: HashMap<String, Bound<'py, PyAny>> = HashMap::new();
    map.insert(
        "font_family".into(),
        style
            .font
            .as_ref()
            .map(|f| f.family_name.as_str())
            .into_pyobject(py)?,
    );
    map.insert(
        "color".into(),
        style
            .color
            .as_ref()
            .map(|c| c.to_string())
            .into_pyobject(py)?,
    );
    map.insert(
        "size".into(),
        style.size.map(|x| x.get()).into_pyobject(py)?,
    );
    map.insert(
        "line_spacing".into(),
        style.line_spacing.map(|x| x.get()).into_pyobject(py)?,
    );
    map.insert("italic".into(), style.italic.into_pyobject(py)?);
    map.insert("stretch".into(), stretch_idx.into_pyobject(py)?);
    map.insert("weight".into(), style.weight.into_pyobject(py)?);
    map.insert("underline".into(), style.underline.into_pyobject(py)?);
    map.insert("line_through".into(), style.line_through.into_pyobject(py)?);
    map.into_pyobject(py)
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyTextStyleOrName {
    Name(String),
    Style(ValueOrInSteps<PyTextStyle>),
}
