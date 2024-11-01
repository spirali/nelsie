use crate::model::{PartialTextStyle, Step, StepValue, TextStyle};

pub(crate) type InTextBoxId = u32;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct InTextAnchor {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct StyledRange {
    pub start: u32,
    pub end: u32,
    pub style: PartialTextStyle,
}

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub(crate) struct StyledText {
    pub text: String,
    pub main_style: TextStyle,
    pub styles: Vec<StyledRange>,
    pub anchors: Vec<(InTextBoxId, InTextAnchor)>,
    pub text_align: TextAlign,
}

impl StyledText {
    pub fn new_simple_text(text: String, main_style: TextStyle) -> Self {
        StyledText {
            text,
            main_style,
            styles: Vec::new(),
            anchors: Default::default(),
            text_align: TextAlign::Start,
        }
    }

    pub fn replace_text(&mut self, value1: &str, value2: &str) {
        while let Some(start_idx) = self.text.find(value1) {
            self.text = self.text.replace(value1, value2);
            let start_idx = start_idx as u32;
            let idx = start_idx + value1.len() as u32;
            for style in self.styles.iter_mut() {
                if style.start >= start_idx && style.start <= idx {
                    style.start = start_idx;
                } else if style.start >= idx {
                    style.start += value2.len() as u32;
                    style.start -= value1.len() as u32;
                }
                if style.end >= start_idx && style.end <= idx {
                    style.end = idx + value2.len() as u32;
                } else if style.end >= idx {
                    style.end += value2.len() as u32;
                    style.end -= value1.len() as u32;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum TextAlign {
    Start,
    Center,
    End,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Start
    }
}

#[derive(Debug)]
pub(crate) struct NodeContentText {
    pub styled_text: StepValue<StyledText>,
    pub parse_counters: bool,
}

impl NodeContentText {
    pub fn styled_text_at_step(&self, step: &Step) -> &StyledText {
        self.styled_text.at_step(step)
    }
}
