use crate::model::{LayoutExpr, Length, LengthOrAuto, NodeId};
use crate::parsers::StringOrFloat;
use std::str::FromStr;

fn parse<T: FromStr>(s: &str, value: &str) -> crate::Result<T> {
    s.parse()
        .map_err(|_| crate::NelsieError::ParsingError(format!("Invalid value: {value}")))
}

pub(crate) fn parse_length(value: StringOrFloat) -> crate::Result<Length> {
    match value {
        StringOrFloat::Float(value) => Ok(Length::Points { value }),
        StringOrFloat::String(str) => Ok(if let Some(s) = str.trim().strip_suffix("%") {
            Length::Fraction {
                value: parse::<f32>(s, &str)? / 100.0,
            }
        } else {
            Length::Points {
                value: parse::<f32>(&str, &str)?,
            }
        }),
    }
}

pub(crate) fn parse_length_auto(value: StringOrFloat) -> crate::Result<LengthOrAuto> {
    match value {
        StringOrFloat::Float(value) => Ok(LengthOrAuto::Points { value }),
        StringOrFloat::String(str) if str.trim() == "auto" => Ok(LengthOrAuto::Auto),
        StringOrFloat::String(str) => Ok(if let Some(s) = str.trim().strip_suffix("%") {
            LengthOrAuto::Fraction {
                value: parse::<f32>(s, &str)? / 100.0,
            }
        } else {
            LengthOrAuto::Points {
                value: parse::<f32>(&str, &str)?,
            }
        }),
    }
}

pub(crate) fn parse_position(
    parent_id: NodeId,
    value: StringOrFloat,
    is_x: bool,
) -> crate::Result<LayoutExpr> {
    Ok(match value {
        StringOrFloat::Float(v) => LayoutExpr::Sum {
            expressions: vec![
                if is_x {
                    LayoutExpr::X { node_id: parent_id }
                } else {
                    LayoutExpr::Y { node_id: parent_id }
                },
                LayoutExpr::ConstValue { value: v },
            ],
        },
        StringOrFloat::String(v) => todo!(),
    })
}

#[cfg(test)]
mod tests {
    use crate::model::Length;
    use crate::parsers::{parse_length, StringOrFloat};

    #[test]
    fn test_parse_length() {
        let s = |s: &str| StringOrFloat::String(s.to_string());
        assert_eq!(
            parse_length(s("213")).unwrap(),
            Length::Points { value: 213.0 }
        );
        assert_eq!(
            parse_length(s("2.5")).unwrap(),
            Length::Points { value: 2.5 }
        );
        assert_eq!(parse_length(s("0")).unwrap(), Length::Points { value: 0.0 });
        assert_eq!(
            parse_length(s("95%")).unwrap(),
            Length::Fraction { value: 0.95 }
        );
        assert_eq!(
            parse_length(s("0%")).unwrap(),
            Length::Fraction { value: 0.0 }
        );
    }
}
