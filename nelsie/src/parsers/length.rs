use renderer::Length;
use std::str::FromStr;

fn parse<T: FromStr>(value: &str, full_value: &str) -> crate::Result<T> {
    T::from_str(value).map_err(|_| crate::Error::generic_err(format!("Cannot parse: {full_value}")))
}

pub(crate) fn parse_string_length(value: &str) -> crate::Result<Length> {
    Ok(if let Some(s) = value.trim().strip_suffix('%') {
        Length::Fraction {
            value: parse::<f32>(s, value)? / 100.0,
        }
    } else {
        Length::Points {
            value: parse::<f32>(value, value)?,
        }
    })
}
