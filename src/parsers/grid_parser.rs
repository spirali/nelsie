use crate::common::error::NelsieError;
use crate::parsers::parse_utils::{parse_int, parse_u32, parse_uint, CharParser};
use crate::parsers::{StringOrFloat, StringOrInt};
use chumsky::prelude::just;
use chumsky::text::TextParser;
use chumsky::Parser;
use taffy::prelude::{FromFlex, FromLength, FromPercent, TaffyAuto, TaffyGridLine, TaffyGridSpan};
use taffy::{GridPlacement, NonRepeatedTrackSizingFunction};

pub(crate) fn parse_grid_template_item(
    value: StringOrFloat,
) -> crate::Result<NonRepeatedTrackSizingFunction> {
    match value {
        StringOrFloat::Float(v) => Ok(NonRepeatedTrackSizingFunction::from_length(v)),
        StringOrFloat::String(s) => p_grid_template()
            .parse_text(&s)
            .map_err(|_| NelsieError::generic_err("Invalid grid template definition")),
    }
}

pub(crate) fn parse_grid_position_item(value: StringOrInt<i16>) -> crate::Result<GridPlacement> {
    match value {
        StringOrInt::Int(x) => Ok(GridPlacement::from_line_index(x)),
        StringOrInt::String(s) => p_grid_position()
            .parse_text(&s)
            .map_err(|_| NelsieError::generic_err("Invalid grid position")),
    }
}

fn p_grid_template() -> impl CharParser<NonRepeatedTrackSizingFunction> {
    parse_u32() // TODO: parse_f32()
        .then(just("%").padded().or(just("fr").padded()).or_not())
        .map(|(v, t)| match t {
            None => NonRepeatedTrackSizingFunction::from_length(v as f32),
            Some("%") => NonRepeatedTrackSizingFunction::from_percent(v as f32 / 100.0),
            Some("fr") => NonRepeatedTrackSizingFunction::from_flex(v as f32),
            _ => unreachable!(),
        })
}

fn p_grid_position() -> impl CharParser<GridPlacement> {
    just("auto")
        .map(|_| GridPlacement::AUTO)
        .or(just("span")
            .ignore_then(parse_uint::<u16>().padded())
            .map(GridPlacement::from_span))
        .or(parse_int::<i16>().map(GridPlacement::from_line_index))
}

#[cfg(test)]
mod test {
    use crate::parsers::grid_parser::{parse_grid_position_item, parse_grid_template_item};
    use crate::parsers::{StringOrFloat, StringOrInt};
    use taffy::prelude::{FromFlex, FromLength, FromPercent, TaffyGridLine, TaffyGridSpan};
    use taffy::{GridPlacement, NonRepeatedTrackSizingFunction};

    #[test]
    pub fn test_parse_grid_template() {
        assert_eq!(
            parse_grid_template_item(StringOrFloat::Float(23.5)).unwrap(),
            NonRepeatedTrackSizingFunction::from_length(23.5)
        );
        assert_eq!(
            parse_grid_template_item(StringOrFloat::String("12".to_string())).unwrap(),
            NonRepeatedTrackSizingFunction::from_length(12.0)
        );
        assert_eq!(
            parse_grid_template_item(StringOrFloat::String("45%".to_string())).unwrap(),
            NonRepeatedTrackSizingFunction::from_percent(0.45)
        );
        assert_eq!(
            parse_grid_template_item(StringOrFloat::String("2 fr".to_string())).unwrap(),
            NonRepeatedTrackSizingFunction::from_flex(2.0)
        );
    }

    #[test]
    pub fn test_parse_grid_position() {
        assert_eq!(
            parse_grid_position_item(StringOrInt::<i16>::Int(-2)).unwrap(),
            GridPlacement::from_line_index(-2)
        );
        assert_eq!(
            parse_grid_position_item(StringOrInt::<i16>::String("-12".to_string())).unwrap(),
            GridPlacement::from_line_index(-12)
        );
        assert_eq!(
            parse_grid_position_item(StringOrInt::<i16>::String("0".to_string())).unwrap(),
            GridPlacement::from_line_index(0)
        );
        assert_eq!(
            parse_grid_position_item(StringOrInt::<i16>::String("auto".to_string())).unwrap(),
            GridPlacement::Auto
        );
        assert_eq!(
            parse_grid_position_item(StringOrInt::<i16>::String("span 5".to_string())).unwrap(),
            GridPlacement::from_span(5)
        );
    }
}
