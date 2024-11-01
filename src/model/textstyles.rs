use crate::common::error::NelsieError;
use crate::model::StepValue;

use crate::common::Color;
use resvg::usvg::PositiveF32;
use std::collections::HashMap;
use std::sync::Arc;
use svg2pdf::usvg;
use usvg::FontStretch;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct FontData {
    pub family_name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PartialTextStyle {
    pub font: Option<Arc<FontData>>,
    pub color: Option<Color>,
    pub size: Option<PositiveF32>,
    pub line_spacing: Option<PositiveF32>,
    pub italic: Option<bool>,
    pub stretch: Option<FontStretch>,
    pub weight: Option<u16>,
    pub underline: Option<bool>,
    pub line_through: Option<bool>,
}

impl PartialTextStyle {
    pub fn into_text_style(self) -> Option<TextStyle> {
        Some(TextStyle {
            font: self.font?,
            color: self.color?,
            size: self.size?,
            line_spacing: self.line_spacing?,
            italic: self.italic?,
            stretch: self.stretch?,
            weight: self.weight?,
            underline: self.underline?,
            line_through: self.line_through?,
        })
    }

    pub fn merge(&self, other: &PartialTextStyle) -> PartialTextStyle {
        PartialTextStyle {
            font: other.font.as_ref().or(self.font.as_ref()).cloned(),
            color: other.color.or(self.color),
            size: other.size.or(self.size),
            line_spacing: other.line_spacing.or(self.line_spacing),
            italic: other.italic.or(self.italic),
            stretch: other.stretch.or(self.stretch),
            weight: other.weight.or(self.weight),
            underline: other.underline.or(self.underline),
            line_through: other.line_through.or(self.line_through),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TextStyle {
    pub font: Arc<FontData>,
    pub color: Color,
    pub size: PositiveF32,
    pub line_spacing: PositiveF32,
    pub italic: bool,
    pub stretch: FontStretch,
    pub weight: u16,
    pub underline: bool,
    pub line_through: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            font: Arc::new(FontData {
                family_name: "".to_string(),
            }),
            color: Color::default(),
            size: PositiveF32::ZERO,
            line_spacing: PositiveF32::ZERO,
            italic: false,
            stretch: Default::default(),
            weight: 0,
            underline: false,
            line_through: false,
        }
    }
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

    pub fn set_style(&mut self, name: String, style: StepValue<PartialTextStyle>) {
        if name == "default" {
            // This prevents to get empty "holes" into default style
            self.update_style(name, style)
        } else {
            self.0.insert(name, style);
        }
    }

    pub fn update_style(&mut self, name: String, mut style: StepValue<PartialTextStyle>) {
        style = self
            .0
            .get(&name)
            .map(|s| s.merge(&style, |s, t| s.merge(t)))
            .unwrap_or(style);
        self.0.insert(name, style);
    }

    pub fn get_style(&self, name: &str) -> crate::Result<&StepValue<PartialTextStyle>> {
        self.0
            .get(name)
            .ok_or_else(|| NelsieError::generic_err(format!("Style '{name}' not found")))
    }
}
