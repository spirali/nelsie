use std::str::FromStr;
use crate::model::Color;

pub(crate) fn parse_color(value: &str) -> crate::Result<Color> {
    Ok(Color::new(svgtypes::Color::from_str(&value)
        .map_err(|_| crate::NelsieError::ParsingError(format!("Invalid color: '{value}'")))?))
    }