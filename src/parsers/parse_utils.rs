use crate::common::error::NelsieError;
use chumsky::error::Simple;
use chumsky::prelude::just;
use chumsky::primitive::end;
use chumsky::text::int;
use chumsky::Parser;
use std::str::FromStr;

pub(crate) type ParseError = Simple<char>;
//
// #[derive(Clone, Debug, PartialEq)]
// pub(crate) struct ParseError {
//     error: Simple<String>,
// }
// impl ParseError {
//     pub fn custom<M: ToString>(span: <Self as Error<char>>::Span, msg: M) -> Self {
//         Self {
//             error: Simple::custom(span, msg),
//         }
//     }
// }
//
// impl Error<char> for ParseError {
//     type Span = Range<usize>;
//     type Label = &'static str;
//
//     fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
//         span: Self::Span,
//         expected: Iter,
//         found: Option<char>,
//     ) -> Self {
//         let expected = expected.into_iter().map(|c| c.map(|c| c.to_string()));
//         Self {
//             error: Simple::expected_input_found(span, expected, found.map(|c| c.to_string())),
//         }
//     }
//
//     fn with_label(self, label: Self::Label) -> Self {
//         Self {
//             error: self.error.with_label(label),
//         }
//     }
//
//     fn merge(self, other: Self) -> Self {
//         let merged = self.error.merge(other.error);
//         Self { error: merged }
//     }
// }

pub(crate) trait CharParser<T>: Parser<char, T, Error = ParseError> + Sized + Clone {
    fn parse_text(&self, input: &str) -> crate::Result<T> {
        self.then_ignore(end())
            .parse(input.trim())
            .map_err(|e| NelsieError::Parsing(e[0].to_string()))
    }
}

impl<T, P> CharParser<T> for P where P: Parser<char, T, Error = ParseError> + Clone {}

pub fn parse_u32() -> impl CharParser<u32> {
    parse_uint::<u32>()
}

pub fn parse_uint<T: FromStr>() -> impl CharParser<T> {
    int(10).try_map(|v: String, span| {
        v.parse()
            .map_err(|_| ParseError::custom(span, "Invalid int"))
    })
}

pub fn parse_int<T: FromStr + std::ops::Neg<Output = T>>() -> impl CharParser<T> {
    just('-').or_not().then(int(10)).try_map(|(sign, v), span| {
        let r: T = v
            .parse()
            .map_err(|_| ParseError::custom(span, "Invalid int"))?;
        Ok(if sign.is_none() { r } else { r.neg() })
    })
}

#[cfg(test)]
mod test {
    use super::parse_int;
    use crate::parsers::parse_utils::CharParser;

    #[test]
    pub fn test_parse_int() {
        assert_eq!(parse_int::<i16>().parse_text("12").unwrap(), 12);
        assert_eq!(parse_int::<i16>().parse_text("-3").unwrap(), -3);
    }
}
