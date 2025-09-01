use crate::Color;
use resvg::usvg::PositiveF32;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;

pub use resvg::usvg::FontStretch;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct InlineId(u32);

impl InlineId {
    pub fn new(text_id: u32) -> Self {
        InlineId(text_id)
    }
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl FromStr for InlineId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).map(InlineId::new)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextStyle {
    pub font: Option<Arc<String>>,
    pub color: Option<Color>,
    pub size: Option<PositiveF32>,
    pub line_spacing: Option<PositiveF32>,
    pub italic: Option<bool>,
    pub stretch: Option<FontStretch>,
    pub weight: Option<u16>,
    pub underline: Option<bool>,
    pub line_through: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxHighlightSettings {
    pub language: String,
    pub theme: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ParsingChars {
    pub escape_char: char,
    pub block_begin: char,
    pub block_end: char,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextStyling {
    pub parsing_chars: ParsingChars,
    pub named_styles: Vec<(String, TextStyle)>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Text {
    pub text: String,
    pub style: TextStyle,
    pub styling: Option<TextStyling>,
    pub text_align: TextAlign,
    pub syntax_highlight: Option<SyntaxHighlightSettings>,
}
