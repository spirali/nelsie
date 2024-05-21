use crate::model::{Color, PartialTextStyle, Resources, Stroke};

use crate::pyinterface::insteps::ValueOrInSteps;
use pyo3::exceptions::PyValueError;
use pyo3::pybacked::PyBackedStr;
use pyo3::{FromPyObject, PyAny, PyObject, PyResult, Python, ToPyObject};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use svg2pdf::usvg;
use usvg::FontStretch;

#[derive(Debug, Default)]
pub(crate) struct PyTextStyle {
    pub font_family: Option<String>,
    pub stroke: Option<Option<Stroke>>,
    pub color: Option<Option<Color>>,
    pub size: Option<f32>,
    pub line_spacing: Option<f32>,
    pub italic: Option<bool>,
    pub stretch: Option<FontStretch>,
    pub weight: Option<u16>,
    pub underline: Option<bool>,
    pub overline: Option<bool>,
    pub line_through: Option<bool>,
}

impl PyTextStyle {
    // pub fn new(style: PartialTextStyle) -> Self {
    //     PyTextStyle({
    //                 }
    // }
    pub fn into_partial_style(self, resources: &Resources) -> crate::Result<PartialTextStyle> {
        let font = self
            .font_family
            .map(|name| resources.check_font(&name).map(Arc::new))
            .transpose()?;
        Ok(PartialTextStyle {
            font,
            stroke: self.stroke.map(|s| s.map(Arc::new)),
            color: self.color,
            size: self.size,
            line_spacing: self.line_spacing,
            italic: self.italic,
            stretch: self.stretch,
            weight: self.weight,
            underline: self.underline,
            overline: self.overline,
            line_through: self.line_through,
        })
    }
}

impl<'py> FromPyObject<'py> for Color {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        let s: PyBackedStr = ob.extract()?;
        Ok(Color::from_str(s.deref())?)
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
            .extract::<Option<PyBackedStr>>()?
            .map(|c| -> PyResult<_> {
                if c.trim() == "empty" {
                    Ok(None)
                } else {
                    Ok(Some(Color::from_str(c.deref())?))
                }
            })
            .transpose()?;
        let stroke_attr = ob.getattr("stroke")?;
        let stroke = if let Ok(s) = stroke_attr.extract::<PyBackedStr>() {
            if s.deref() == "empty" {
                Some(None)
            } else {
                return Err(PyValueError::new_err("Invalid stroke value"));
            }
        } else {
            stroke_attr.extract::<Option<Stroke>>()?.map(Some)
        };
        Ok(PyTextStyle {
            font_family: ob.getattr("font_family")?.extract::<Option<String>>()?,
            stroke,
            color,
            size: ob.getattr("size")?.extract()?,
            line_spacing: ob.getattr("line_spacing")?.extract()?,
            italic: ob.getattr("italic")?.extract()?,
            stretch,
            weight: ob.getattr("weight")?.extract()?,
            underline: ob.getattr("underline")?.extract()?,
            overline: ob.getattr("overline")?.extract()?,
            line_through: ob.getattr("line_through")?.extract()?,
        })
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

pub(crate) fn partial_text_style_to_pyobject(style: &PartialTextStyle, py: Python<'_>) -> PyObject {
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
    let mut map: HashMap<String, PyObject> = HashMap::new();
    map.insert(
        "font_family".into(),
        style
            .font
            .as_ref()
            .map(|f| f.family_name.as_str())
            .to_object(py),
    );
    map.insert(
        "color".into(),
        style
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
        style
            .stroke
            .as_ref()
            .map(|v| {
                v.as_ref()
                    .map(|s| stroke_to_py_map(s, py))
                    .unwrap_or_else(|| "empty".to_object(py))
            })
            .to_object(py),
    );
    map.insert("size".into(), style.size.to_object(py));
    map.insert("line_spacing".into(), style.line_spacing.to_object(py));
    map.insert("italic".into(), style.italic.to_object(py));
    map.insert("stretch".into(), stretch_idx.to_object(py));
    map.insert("weight".into(), style.weight.to_object(py));
    map.insert("underline".into(), style.underline.to_object(py));
    map.insert("overline".into(), style.overline.to_object(py));
    map.insert("line_through".into(), style.line_through.to_object(py));
    map.to_object(py)
}

#[derive(Debug, FromPyObject)]
pub(crate) enum PyTextStyleOrName {
    Name(String),
    Style(ValueOrInSteps<PyTextStyle>),
}
