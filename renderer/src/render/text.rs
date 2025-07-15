use crate::Color;
use parley::{FontContext, LayoutContext};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct RenderedText {}

pub(crate) struct TextContext {
    pub layout_cx: LayoutContext<Color>,
    pub font_cx: FontContext,
}
