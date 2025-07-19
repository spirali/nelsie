use crate::text::TextStyle;
use crate::textutils::styling::{StyledRange, StyledText};
use crate::{Color, Resources};
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;

impl From<syntect::highlighting::Color> for Color {
    fn from(value: syntect::highlighting::Color) -> Self {
        Color::new(svgtypes::Color::new_rgba(
            value.r, value.g, value.b, value.a,
        ))
    }
}

fn create_style(s_style: Style) -> TextStyle {
    TextStyle {
        font: None,
        color: Some(s_style.foreground.into()),
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
        underline: if s_style
            .font_style
            .contains(syntect::highlighting::FontStyle::UNDERLINE)
        {
            Some(true)
        } else {
            None
        },
        line_through: None,
    }
}

pub fn run_syntax_highlighting(
    resources: &Resources,
    text: &mut StyledText,
    language_name: &str,
    theme_name: &str,
) -> crate::Result<()> {
    let syntax = resources
        .syntax_set
        .find_syntax_by_name(language_name)
        .or_else(|| resources.syntax_set.find_syntax_by_extension(language_name))
        .ok_or_else(|| {
            crate::Error::generic_err(format!(
                "Language '{language_name}' for syntax highlighting not found"
            ))
        })?;
    let theme = resources
        .theme_set
        .themes
        .get(theme_name)
        .ok_or_else(|| crate::Error::generic_err(format!("Theme '{theme_name}' not found")))?;
    let mut highlight = HighlightLines::new(syntax, theme);
    let mut styles = Vec::new();
    let mut offset = 0;
    for line in &mut text.text.lines() {
        let highlighted_line = highlight
            .highlight_line(line, &resources.syntax_set)
            .map_err(|e| crate::Error::generic_err(format!("Syntax highlight error: {}", e)))?;
        for (style, word) in highlighted_line {
            let len = word.len() as u32;
            styles.push(StyledRange {
                start: offset,
                end: offset + len,
                style: create_style(style),
            });
            offset += len;
        }
        offset += 1;
    }
    styles.append(&mut text.styles);
    text.styles = styles;
    Ok(())
}
