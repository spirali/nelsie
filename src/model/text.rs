use crate::model::{PartialTextStyle, Step, StepValue, TextStyle};
use std::collections::HashMap;

pub(crate) type InTextBoxId = u32;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct InTextAnchorPoint {
    pub line_idx: u32,
    pub span_idx: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct InTextAnchor {
    pub start: InTextAnchorPoint,
    pub end: InTextAnchorPoint,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct Span {
    pub length: u32,
    pub style_idx: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct StyledLine {
    pub spans: Vec<Span>,
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct StyledText {
    pub styled_lines: Vec<StyledLine>,
    pub main_style: TextStyle,
    pub styles: Vec<PartialTextStyle>,
    pub anchors: HashMap<InTextBoxId, InTextAnchor>,
}

impl StyledText {
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

#[derive(Debug)]
pub(crate) struct NodeContentText {
    pub styled_text: StepValue<StyledText>,
    pub text_align: TextAlign,
    pub parse_counters: bool,
}

impl NodeContentText {
    pub fn styled_text_at_step(&self, step: &Step) -> &StyledText {
        self.styled_text.at_step(step)
    }
}
