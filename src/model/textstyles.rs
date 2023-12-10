use crate::common::error::NelsieError;
use crate::model::{Color, StepValue};
use pyo3::ffi::PyAsyncGen_Type;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use usvg_tree::Text;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct PartialTextStyle {
    pub font_family: Option<Arc<String>>,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub line_spacing: Option<f32>,
}

impl PartialTextStyle {
    pub fn into_text_style(self) -> Option<TextStyle> {
        Some(TextStyle {
            font_family: self.font_family?,
            color: self.color?,
            size: self.size?,
            line_spacing: self.line_spacing?,
        })
    }

    pub fn update(&mut self, other: &PartialTextStyle) {
        let PartialTextStyle {
            font_family,
            color,
            size,
            line_spacing,
        } = other;
        if font_family.is_some() {
            self.font_family = font_family.clone();
        }
        if color.is_some() {
            self.color = color.clone();
        }
        if size.is_some() {
            self.size = *size;
        }
        if line_spacing.is_some() {
            self.line_spacing = *line_spacing;
        }
    }

    pub fn merge(&self, other: &PartialTextStyle) -> PartialTextStyle {
        PartialTextStyle {
            font_family: other
                .font_family
                .as_ref()
                .or(self.font_family.as_ref())
                .cloned(),
            color: other.color.as_ref().or(self.color.as_ref()).cloned(),
            size: other.size.or(self.size),
            line_spacing: other.line_spacing.or(self.line_spacing),
        }
    }

    pub fn from_text_style(text_style: &TextStyle) -> Self {
        Self {
            font_family: Some(text_style.font_family.clone()),
            color: Some(text_style.color.clone()),
            size: Some(text_style.size),
            line_spacing: Some(text_style.line_spacing),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextStyle {
    pub font_family: Arc<String>,
    pub color: Color,
    pub size: f32,
    pub line_spacing: f32,
}

impl TextStyle {
    /*
    pub fn update(&mut self, other: &PartialTextStyle) {
        let PartialTextStyle {
            font_family,
            color,
            size,
            line_spacing,
        } = other;
        if let Some(value) = font_family {
            self.font_family = value.clone();
        }
        TextStyle {
            font_family: partial_style
                .font_family
                .as_ref()
                .unwrap_or(&self.font_family)
                .clone(),
            color: partial_style.color.as_ref().unwrap_or(&self.color).clone(),
            size: partial_style.size.unwrap_or(self.size),
            line_spacing: partial_style.line_spacing.unwrap_or(self.line_spacing),
        }
    }*/
}

pub(crate) fn merge_stepped_styles(
    first: &StepValue<PartialTextStyle>,
    second: &StepValue<PartialTextStyle>,
) -> StepValue<PartialTextStyle> {
    first.merge(second, |a, b| a.merge(b))
}

#[derive(Debug, Default, Clone)]
pub(crate) struct StyleMap(HashMap<String, StepValue<PartialTextStyle>>);

impl StyleMap {
    pub fn new(map: HashMap<String, StepValue<PartialTextStyle>>) -> Self {
        StyleMap(map)
    }

    pub fn set_style(&mut self, name: String, mut style: StepValue<PartialTextStyle>) {
        if name == "default" {
            style = self.0.get(&name).unwrap().merge(&style, |s, t| {
                let mut s = s.clone();
                s.update(t);
                s
            })
        }
        self.0.insert(name, style);
    }

    pub fn get_style(&self, name: &str) -> crate::Result<&StepValue<PartialTextStyle>> {
        self.0
            .get(name)
            .ok_or_else(|| NelsieError::generic_err(format!("Style '{name}' not found")))
    }
}
