use crate::common::error::NelsieError;
use crate::model::{Color, StepValue};

use std::collections::HashMap;
use std::sync::Arc;
use usvg_tree::FontStretch;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct PartialTextStyle {
    pub font_family: Option<Arc<String>>,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub line_spacing: Option<f32>,
    pub italic: Option<bool>,
    pub stretch: Option<FontStretch>,
    pub weight: Option<u16>,
}

impl PartialTextStyle {
    pub fn into_text_style(self) -> Option<TextStyle> {
        Some(TextStyle {
            font_family: self.font_family?,
            color: self.color?,
            size: self.size?,
            line_spacing: self.line_spacing?,
            italic: self.italic?,
            stretch: self.stretch?,
            weight: self.weight?,
        })
    }

    pub fn update(&mut self, other: &PartialTextStyle) {
        let PartialTextStyle {
            font_family,
            color,
            size,
            line_spacing,
            italic,
            stretch,
            weight,
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
        if italic.is_some() {
            self.italic = *italic;
        }
        if stretch.is_some() {
            self.stretch = *stretch;
        }
        if weight.is_some() {
            self.weight = *weight;
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
            italic: other.italic.or(self.italic),
            stretch: other.stretch.or(self.stretch),
            weight: other.weight.or(self.weight),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextStyle {
    pub font_family: Arc<String>,
    pub color: Color,
    pub size: f32,
    pub line_spacing: f32,
    pub italic: bool,
    pub stretch: FontStretch,
    pub weight: u16,
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
