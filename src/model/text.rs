use serde::Deserialize;
use crate::model::Color;

#[derive(Debug, Deserialize)]
pub(crate) struct TextStyle {
    pub color: Color,
    pub size: f32,
    pub line_spacing: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Span {
    pub start: u32,
    pub length: u32,
    pub style_idx: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StyledLine {
    pub spans: Vec<Span>,
    pub text: String,
}

impl StyledLine {
    pub fn line_height(&self, text_styles: &[TextStyle]) -> Option<f32> {
        self.spans.iter().map(|span| {
            let style = &text_styles[span.style_idx as usize];
            style.size * style.line_spacing
        }).max_by(|x, y| x.partial_cmp(y).unwrap())
    }
    pub fn font_size(&self, text_styles: &[TextStyle]) -> Option<f32> {
        self.spans.iter().map(|span| {
            let style = &text_styles[span.style_idx as usize];
            style.size
        }).max_by(|x, y| x.partial_cmp(y).unwrap())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct StyledText {
    pub styled_lines: Vec<StyledLine>,
    pub styles: Vec<TextStyle>,
    pub default_font_size: f32,
    pub default_line_spacing: f32,
}

impl StyledText {
    pub fn height(&self) -> f32 {
        self.styled_lines.iter().map(|line| line.line_height(&self.styles).unwrap_or_else(|| self.default_line_height())).sum()
    }

    pub fn font_size(&self, line_idx: usize) -> f32 {
        self.styled_lines.get(line_idx).and_then(|line| line.font_size(&self.styles)).unwrap_or(self.default_font_size)
    }

    pub fn default_line_height(&self) -> f32 {
        self.default_line_spacing * self.default_font_size
    }

    pub fn line_height(&self, line_idx: usize) -> f32 {
        self.styled_lines.get(line_idx).and_then(|line| line.line_height(&self.styles)).unwrap_or_else(|| self.default_line_height())
    }
}