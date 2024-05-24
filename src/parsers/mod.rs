mod parse_utils;
mod size;
pub(crate) mod step_parser;
mod sxml;
mod syntaxhighlight;
mod text;

use crate::model::LayoutExpr;

#[derive(Debug)]
pub(crate) enum StringOrFloat {
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub(crate) enum StringOrFloatOrExpr {
    Float(f32),
    String(String),
    Expr(LayoutExpr),
}

pub(crate) use size::{parse_length, parse_length_auto, parse_length_or_expr, parse_position};
pub(crate) use sxml::SimpleXmlWriter;
pub(crate) use syntaxhighlight::run_syntax_highlighting;
pub(crate) use text::{parse_styled_text, parse_styled_text_from_plain_text, StyleOrName};
