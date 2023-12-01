mod color;
mod size;

#[derive(Debug)]
pub(crate) enum StringOrFloat {
    Float(f32),
    String(String),
}

pub(crate) use color::parse_color;
pub(crate) use size::{parse_length, parse_position, parse_length_auto};
