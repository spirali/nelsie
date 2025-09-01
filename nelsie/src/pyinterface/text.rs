use crate::pyinterface::common::PyColor;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyErr, PyResult};
use renderer::{
    FontStretch, ParsingChars, SyntaxHighlightSettings, Text, TextAlign, TextStyle, TextStyling,
};
use std::collections::HashMap;
use std::sync::Arc;
use strict_num::PositiveF32;

#[derive(FromPyObject)]
pub(crate) struct PyTextStyle {
    font: Option<String>,
    color: Option<PyColor>,
    size: Option<f32>,
    line_spacing: Option<f32>,
    italic: Option<bool>,
    stretch: Option<u8>,
    underline: Option<bool>,
    line_through: Option<bool>,
    weight: Option<u16>,
    bold: Option<bool>,
}

impl TryFrom<PyTextStyle> for TextStyle {
    type Error = PyErr;

    fn try_from(value: PyTextStyle) -> Result<Self, Self::Error> {
        let size = value
            .size
            .map(|s| PositiveF32::new(s).ok_or_else(|| PyValueError::new_err("Invalid font size")))
            .transpose()?;
        let line_spacing = value
            .line_spacing
            .map(|s| {
                PositiveF32::new(s)
                    .ok_or_else(|| PyValueError::new_err("Invalid line spacing size"))
            })
            .transpose()?;
        let stretch = value
            .stretch
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

        let weight = if value.bold == Some(true) {
            Some(700)
        } else {
            value.weight
        };

        Ok(TextStyle {
            font: value.font.map(Arc::new),
            color: value.color.map(|x| x.into()),
            size,
            line_spacing,
            italic: value.italic,
            stretch,
            weight,
            underline: value.underline,
            line_through: value.line_through,
        })
    }
}

// """
//     text: Sv[str]
//     style: Sn[TextStyle]
//     align: Sv[TextAlign]
//     syntax_language: Sn[str]
//     syntax_theme: Sn[str]
// """

pub(crate) struct PyTextAlign(TextAlign);

impl<'py> FromPyObject<'py> for PyTextAlign {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = ob.extract()?;
        Ok(PyTextAlign(match s {
            "start" => TextAlign::Start,
            "center" => TextAlign::Center,
            "end" => TextAlign::End,
            _ => return Err(PyValueError::new_err(format!("Invalid text align: '{s}'"))),
        }))
    }
}

impl From<PyTextAlign> for TextAlign {
    fn from(value: PyTextAlign) -> Self {
        value.0
    }
}

#[derive(FromPyObject)]
pub(crate) struct PyTextContent {
    pub(crate) text: String,
    pub(crate) style: PyTextStyle,
    pub(crate) align: PyTextAlign,
    pub(crate) syntax_language: Option<String>,
    pub(crate) syntax_theme: Option<String>,
    pub(crate) named_styles: Option<HashMap<String, PyTextStyle>>,
    pub(crate) style_delimiters: Option<String>,
}

impl TryFrom<PyTextContent> for Text {
    type Error = PyErr;

    fn try_from(value: PyTextContent) -> Result<Self, Self::Error> {
        let style = value.style.try_into()?;
        let syntax_highlight =
            if let (Some(language), Some(theme)) = (value.syntax_language, value.syntax_theme) {
                Some(SyntaxHighlightSettings { language, theme })
            } else {
                None
            };
        Ok(Text {
            text: value.text,
            style,
            styling: if let Some(style_delimiters) = value.style_delimiters {
                if style_delimiters.len() != 3 {
                    return Err(PyValueError::new_err(
                        "Style delimiters must be 3 characters long",
                    ));
                }
                let mut chars = style_delimiters.chars();
                let parsing_chars = ParsingChars {
                    escape_char: chars.next().unwrap(),
                    block_begin: chars.next().unwrap(),
                    block_end: chars.next().unwrap(),
                };
                let mut named_styles: Vec<_> = value
                    .named_styles
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(k, v)| {
                        let v = v.try_into()?;
                        PyResult::Ok((k, v))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                named_styles.sort_unstable_by(|a, b| a.0.cmp(&b.0));
                Some(TextStyling {
                    parsing_chars,
                    named_styles,
                })
            } else {
                None
            },
            text_align: value.align.into(),
            syntax_highlight,
        })
    }
}
