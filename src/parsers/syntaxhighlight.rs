use crate::common::error::NelsieError;
use crate::model::{Color, NodeContentText, Resources, Span, TextStyle};

use syntect::easy::HighlightLines;
use syntect::highlighting::Style;

impl From<syntect::highlighting::Color> for Color {
    fn from(value: syntect::highlighting::Color) -> Self {
        Color::new(svgtypes::Color::new_rgba(
            value.r, value.g, value.b, value.a,
        ))
    }
}

fn update_style(style: &mut TextStyle, s_style: Style) {
    style.color = Some(s_style.foreground.into());
    if s_style
        .font_style
        .contains(syntect::highlighting::FontStyle::BOLD)
    {
        style.weight = 700;
    }
    if s_style
        .font_style
        .contains(syntect::highlighting::FontStyle::ITALIC)
    {
        style.italic = true;
    }
}

pub fn run_syntax_highlighting(
    resources: &Resources,
    text: &mut NodeContentText,
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
            .map_err(|e| {
                NelsieError::generic_err(format!("Syntax highlight error: {}", e))
            })?;
        let mut spans: Vec<Span> = Vec::with_capacity(highlighted_line.len());
        for (style, word) in highlighted_line {
            let style_idx = styles.iter().position(|s| s == &style).unwrap_or_else(|| {
                let idx = styles.len();
                styles.push(style);
                idx
            }) as u32;
            if spans
                .last()
                .map(|span| span.style_idx == style_idx)
                .unwrap_or(false)
            {
                spans.last_mut().unwrap().length += word.len() as u32;
            } else {
                spans.push(Span {
                    length: word.len() as u32,
                    style_idx,
                });
            }
        }
        line.spans = spans;
    }
    let style = &text.styles[0];
    text.styles = styles
        .into_iter()
        .map(|s| {
            let style = style.clone();
            style.map(|mut t| {
                update_style(&mut t, s);
                t
            })
        })
        .collect();
    Ok(())
}
