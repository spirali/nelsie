use std::str::FromStr;
use crate::model::Length;

fn parse<T : FromStr>(s: &str, value: &str) -> crate::Result<T> {
    s.parse().map_err(|_| crate::NelsieError::ParsingError(format!("Invalid value: {value}")))
}

pub(crate) fn parse_length(str: &str) -> crate::Result<Length> {
    Ok(if let Some(s) = str.trim().strip_suffix("%") {
        Length::Fraction {value: parse::<f32>(s, str)? / 100.0}
    } else {
        Length::Points {value: parse::<f32>(str, str)?}
    })
}

#[cfg(test)]
mod tests {
    use crate::model::Length;
    use crate::parsers::parse_length;

    #[test]
    fn test_parse_length() {
        assert_eq!(parse_length("213").unwrap(), Length::Points { value: 213.0});
        assert_eq!(parse_length("2.5").unwrap(), Length::Points { value: 2.5});
        assert_eq!(parse_length("0").unwrap(), Length::Points { value: 0.0});
        assert_eq!(parse_length("95%").unwrap(), Length::Fraction { value: 0.95});
        assert_eq!(parse_length("0%").unwrap(), Length::Fraction { value: 0.0});
    }
}