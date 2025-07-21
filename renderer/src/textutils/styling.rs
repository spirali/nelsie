use crate::Resources;
use crate::text::{InlineId, Text, TextAlign, TextStyle};
use crate::textutils::syntaxhl::run_syntax_highlighting;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct StyledRange {
    pub start: u32,
    pub end: u32,
    pub style: TextStyle,
}

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub(crate) struct StyledText {
    pub text: String,
    pub main_style: TextStyle,
    pub styles: Vec<StyledRange>,
    pub anchors: Vec<(InlineId, InlineAnchor)>,
    pub text_align: TextAlign,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct InlineAnchor {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
enum StackEntry<'a> {
    Style { start: u32, name: &'a str },
    Anchor { start: u32, anchor_id: InlineId },
}

fn text_to_styled_text(text: &Text) -> crate::Result<StyledText> {
    if let Some(styling) = &text.styling {
        let mut stack: Vec<StackEntry> = Vec::new();
        let mut input = text.text.as_str();
        let mut result_text = String::with_capacity(input.len());
        let mut result_styles = Vec::new();
        let mut result_anchors = Vec::new();

        let esc_char = styling.parsing_chars.escape_char;
        let block_begin = styling.parsing_chars.block_begin;
        let block_end = styling.parsing_chars.block_end;

        let esc_len = esc_char.len_utf8();
        let start_len = block_begin.len_utf8();
        let end_len = block_end.len_utf8();

        while !input.is_empty() {
            let mut esc_index = input.find(esc_char);
            let mut end_index = if stack.is_empty() {
                None
            } else {
                input.find(block_end)
            };
            if let (Some(idx1), Some(idx2)) = (esc_index, end_index) {
                if idx2 > idx1 {
                    end_index = None;
                } else {
                    esc_index = None;
                }
            }

            if let Some(idx) = esc_index {
                if idx > 0 {
                    result_text.push_str(&input[..idx]);
                }
                let start = idx + esc_len;
                let end = input[start..].find(block_begin).ok_or_else(|| {
                    crate::Error::parsing_err(format!(
                        "Invalid style formatting: character '{}' found, but no following '{}')",
                        esc_char, block_begin
                    ))
                })? + start;
                let name = &input[start..end];
                if !name.is_empty() && name.chars().all(|x| x.is_ascii_digit()) {
                    stack.push(StackEntry::Anchor {
                        start: result_text.len() as u32,
                        anchor_id: name.parse().unwrap(),
                    });
                } else {
                    stack.push(StackEntry::Style {
                        start: result_text.len() as u32,
                        name,
                    });
                }
                input = &input[end + start_len..];
            } else if let Some(idx) = end_index {
                if idx > 0 {
                    result_text.push_str(&input[..idx]);
                }
                let end = result_text.len() as u32;
                match stack.pop().unwrap() {
                    StackEntry::Style { start, name } => result_styles.push(StyledRange {
                        start,
                        end,
                        style: styling
                            .named_styles
                            .iter()
                            .find(|(n, _)| n.as_str() == name)
                            .ok_or_else(|| {
                                crate::Error::parsing_err(format!("Unknown style: {}", name))
                            })?
                            .1
                            .clone(),
                    }),
                    StackEntry::Anchor { start, anchor_id } => {
                        result_anchors.push((anchor_id, InlineAnchor { start, end }))
                    }
                }
                input = &input[idx + end_len..];
            } else {
                result_text.push_str(input);
                break;
            };
        }

        if !stack.is_empty() {
            return Err(crate::Error::parsing_err("Unclosed style block"));
        }

        result_styles.reverse();
        result_styles.sort_by_key(|s: &StyledRange| (s.start, s.end));

        Ok(StyledText {
            text: result_text,
            main_style: text.style.clone(),
            styles: result_styles,
            anchors: result_anchors,
            text_align: text.text_align,
        })
    } else {
        Ok(StyledText {
            text: text.text.clone(),
            main_style: text.style.clone(),
            styles: Vec::new(),
            anchors: Default::default(),
            text_align: text.text_align,
        })
    }
}

impl StyledText {
    pub fn from(resources: &Resources, text: &Text) -> crate::Result<Self> {
        let mut styled_text = text_to_styled_text(text)?;
        if let Some(hl) = &text.syntax_highlight {
            run_syntax_highlighting(resources, &mut styled_text, &hl.language, &hl.theme)?;
        }
        Ok(styled_text)
    }
}
