use std::collections::HashMap;
use std::sync::Arc;
use resvg::usvg::{FontStretch, PositiveF32};
use crate::Color;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct TextId(u32);

impl TextId {
    pub fn new(text_id: u32) -> Self {
        TextId(text_id)
    }
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct InlineBoxId(u32);

impl InlineBoxId {
    pub fn new(text_id: u32) -> Self {
        InlineBoxId(text_id)
    }
    pub fn as_u32(self) -> u32 {
        self.0
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
    language: String,
    theme: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ParsingChars {
    escape_char: char,
    block_begin: char,
    block_end: char,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextStyling {
    parsing_chars: ParsingChars,
    styles: Vec<(Arc<String>, TextStyle)>
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub(crate) enum TextAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Text {
    pub(crate) text: String,
    pub(crate) style: TextStyle,
    pub(crate) styling: Option<TextStyling>,
    pub(crate) text_align: TextAlign,
    pub(crate) syntax_highlight: Option<SyntaxHighlightSettings>,
}
