mod size;
pub(crate) mod step_parser;
mod text;

#[derive(Debug)]
pub(crate) enum StringOrFloat {
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub(crate) enum StringOrFloatOrExpr {
    Float(f32),
    String(String),
    //Expr(LayoutExpr),
}

pub(crate) use size::{parse_length, parse_length_auto, parse_position};
pub(crate) use text::parse_styled_text;
