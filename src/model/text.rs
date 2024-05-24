use crate::model::{Step, StepValue, TextStyle};
use itertools::Itertools;
use std::collections::HashMap;

pub(crate) type InTextAnchorId = u32;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct InTextAnchorPoint {
    pub line_idx: u32,
    pub span_idx: u32,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct InTextAnchor {
    pub start: InTextAnchorPoint,
    pub end: InTextAnchorPoint,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct Span {
    pub length: u32,
    pub style_idx: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct StyledLine {
    pub spans: Vec<Span>,
    pub text: String,
}

impl StyledLine {
    pub fn line_descender(&self, text_styles: &[TextStyle]) -> Option<f32> {
        self.spans
            .iter()
            .map(|span| {
                let style = &text_styles[span.style_idx as usize];
                style.size * style.font.descender
            })
            .min_by(|x, y| x.partial_cmp(y).unwrap())
    }

    pub fn font_size(&self, text_styles: &[TextStyle]) -> Option<f32> {
        self.spans
            .iter()
            .map(|span| {
                let style = &text_styles[span.style_idx as usize];
                style.size
            })
            .max_by(|x, y| x.partial_cmp(y).unwrap())
    }
}

#[derive(Debug)]
pub(crate) struct StyledText {
    pub styled_lines: Vec<StyledLine>,
    pub styles: Vec<TextStyle>,
    pub default_font_size: f32,
    pub default_line_spacing: f32,
}

impl StyledText {
    pub fn height(&self) -> f32 {
        if self.styled_lines.is_empty() {
            return 0.0;
        }
        self.styled_lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let size = line
                    .font_size(&self.styles)
                    .unwrap_or(self.default_font_size);
                if idx == 0 {
                    size
                } else {
                    size * self.default_line_spacing
                }
            })
            .sum()
    }

    fn replace_line(line: &mut StyledLine, value1: &str, value2: &str) {
        'top: while let Some(target_idx) = line.text.find(value1) {
            let mut idx = 0;
            for span in line.spans.iter_mut() {
                let end = idx + span.length as usize;
                if target_idx >= idx && target_idx + value1.len() <= end {
                    span.length = span.length + value2.len() as u32 - value1.len() as u32;
                    line.text = line.text.replace(value1, value2);
                    continue 'top;
                }
                idx = end;
            }
            break;
        }
    }

    pub fn replace_text(&mut self, value1: &str, value2: &str) {
        for line in &mut self.styled_lines {
            Self::replace_line(line, value1, value2)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TextAlign {
    Start,
    Center,
    End,
}

#[derive(Debug, Default)]
pub(crate) struct ParsedText {
    pub styled_lines: Vec<StyledLine>,
    pub styles: Vec<StepValue<TextStyle>>,
    pub anchors: HashMap<InTextAnchorId, InTextAnchor>,
}

#[derive(Debug)]
pub(crate) struct NodeContentText {
    pub parsed_text: StepValue<ParsedText>,
    pub text_align: TextAlign,
    pub default_font_size: StepValue<f32>,
    pub default_line_spacing: StepValue<f32>,
    pub parse_counters: bool,
}

impl NodeContentText {
    pub fn text_style_at_step(&self, step: &Step) -> StyledText {
        let parsed_text = &self.parsed_text.at_step(step);
        StyledText {
            styled_lines: parsed_text.styled_lines.clone(),
            styles: parsed_text
                .styles
                .iter()
                .map(|s| s.at_step(step).clone())
                .collect_vec(),
            default_font_size: *self.default_font_size.at_step(step),
            default_line_spacing: *self.default_line_spacing.at_step(step),
        }
    }
}
