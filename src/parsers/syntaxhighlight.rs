use crate::common::error::NelsieError;
use crate::model::{Color, NodeContentText, PartialTextStyle, Resources, Span, TextStyle};

use crate::parsers::text::ParsedStyledText;
use crate::parsers::StyleOrName;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;

impl From<syntect::highlighting::Color> for Color {
    fn from(value: syntect::highlighting::Color) -> Self {
        Color::new(svgtypes::Color::new_rgba(
            value.r, value.g, value.b, value.a,
        ))
    }
}

fn create_style(s_style: Style) -> PartialTextStyle {
    PartialTextStyle {
        font: None,
        stroke: None,
        color: Some(Some(s_style.foreground.into())),
        size: None,
        line_spacing: None,
        italic: if s_style
            .font_style
            .contains(syntect::highlighting::FontStyle::ITALIC)
        {
            Some(true)
        } else {
            None
        },
        stretch: None,
        weight: if s_style
            .font_style
            .contains(syntect::highlighting::FontStyle::BOLD)
        {
            Some(700)
        } else {
            None
        },
    }
}

pub fn run_syntax_highlighting(
    resources: &Resources,
    text: &mut ParsedStyledText,
    language_name: &str,
    theme_name: &str,
) -> crate::Result<()> {
    let syntax = resources
        .syntax_set
        .find_syntax_by_extension(language_name)
        .ok_or_else(|| {
            NelsieError::generic_err(format!(
                "Language '{language_name}' for syntax highlighting not found"
            ))
        })?;
    let theme = resources
        .theme_set
        .themes
        .get(theme_name)
        .ok_or_else(|| NelsieError::generic_err(format!("Theme '{theme_name}' not found")))?;
    let mut highlight = HighlightLines::new(syntax, theme);
    let mut styles = Vec::new();
    for line in &mut text.styled_lines {
        let highlighted_line = highlight
            .highlight_line(&line.text, &resources.syntax_set)
            .map_err(|e| NelsieError::generic_err(format!("Syntax highlight error: {}", e)))?;
        let mut spans: Vec<Span> = Vec::with_capacity(highlighted_line.len());
        line.spans.reverse();
        for (style, word) in highlighted_line {
            let mut len = word.len() as u32;
            while len > 0 {
                let last = line.spans.last_mut().unwrap();
                let mut new_style = text.styles[last.style_idx as usize].clone();
                new_style.insert(0, StyleOrName::Style(create_style(style)));
                let style_idx = styles
                    .iter()
                    .position(|s| s == &new_style)
                    .unwrap_or_else(|| {
                        let idx = styles.len();
                        styles.push(new_style);
                        idx
                    }) as u32;
                let span_len = len.min(last.length);
                if spans
                    .last()
                    .map(|span| span.style_idx == style_idx)
                    .unwrap_or(false)
                {
                    spans.last_mut().unwrap().length += span_len;
                } else {
                    spans.push(Span {
                        length: span_len,
                        style_idx,
                    });
                }
                if last.length <= len {
                    len -= last.length;
                    line.spans.pop();
                } else {
                    last.length -= len;
                    len = 0;
                }
            }
        }
        line.spans = spans;
    }
    text.styles = styles;
    Ok(())
}
