use crate::model::{LayoutExpr, Length, LengthOrAuto, LengthOrExpr};
use crate::parsers::{StringOrFloat, StringOrFloatOrExpr};
use std::str::FromStr;

fn parse<T: FromStr>(s: &str, value: &str) -> crate::Result<T> {
    s.parse()
        .map_err(|_| crate::NelsieError::Parsing(format!("Invalid value: {value}")))
}

fn parse_string_length(value: &str) -> crate::Result<Length> {
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

pub(crate) fn parse_length(value: StringOrFloat) -> crate::Result<Length> {
    match value {
        StringOrFloat::Float(value) => Ok(Length::Points { value }),
        StringOrFloat::String(str) => parse_string_length(&str),
    }
}

pub(crate) fn parse_length_auto(value: StringOrFloat) -> crate::Result<LengthOrAuto> {
    match value {
        StringOrFloat::Float(value) => Ok(LengthOrAuto::Points { value }),
        StringOrFloat::String(str) if str.trim() == "auto" => Ok(LengthOrAuto::Auto),
        StringOrFloat::String(str) => Ok(if let Some(s) = str.trim().strip_suffix('%') {
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

pub(crate) fn parse_length_or_expr(value: StringOrFloatOrExpr) -> crate::Result<LengthOrExpr> {
    Ok(match value {
        StringOrFloatOrExpr::Float(value) => LengthOrExpr::Points { value },
        StringOrFloatOrExpr::String(str) => match parse_string_length(&str)? {
            Length::Points { value } => LengthOrExpr::Points { value },
            Length::Fraction { value } => LengthOrExpr::Fraction { value },
        },
        StringOrFloatOrExpr::Expr(expr) => LengthOrExpr::Expr(expr),
    })
}

pub(crate) fn parse_position(value: StringOrFloatOrExpr, is_x: bool) -> crate::Result<LayoutExpr> {
    Ok(match value {
        StringOrFloatOrExpr::Float(v) => {
            if is_x {
                LayoutExpr::ParentX { shift: v }
            } else {
                LayoutExpr::ParentY { shift: v }
            }
        }
        StringOrFloatOrExpr::String(s) => match parse_string_length(&s)? {
            Length::Points { value } => {
                if is_x {
                    LayoutExpr::ParentX { shift: value }
                } else {
                    LayoutExpr::ParentY { shift: value }
                }
            }
            Length::Fraction { value } => {
                if is_x {
                    (LayoutExpr::ParentX { shift: 0.0 })
                        .add(LayoutExpr::ParentWidth { fraction: value })
                } else {
                    (LayoutExpr::ParentY { shift: 0.0 })
                        .add(LayoutExpr::ParentHeight { fraction: value })
                }
            }
        },
        StringOrFloatOrExpr::Expr(expr) => expr,
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
