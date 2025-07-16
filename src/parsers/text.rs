use crate::common::error::NelsieError;
use crate::model::{InTextAnchor, InTextBoxId, PartialTextStyle};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StyleOrName<'a> {
    Name(&'a str),
    Style(PartialTextStyle),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParsedStyleRange<'a> {
    pub start: u32,
    pub end: u32,
    pub style: StyleOrName<'a>,
}

#[derive(Debug)]
pub(crate) struct ParsedStyledText<'a> {
    pub text: String,
    pub styles: Vec<ParsedStyleRange<'a>>,
    pub anchors: Vec<(InTextBoxId, InTextAnchor)>,
}

pub(crate) fn parse_styled_text_from_plain_text(text: &str) -> ParsedStyledText {
    ParsedStyledText {
        text: text.to_string(),
        styles: Vec::new(),
        anchors: Default::default(),
    }
}

#[derive(Debug)]
enum StackEntry<'a> {
    Style { start: u32, name: &'a str },
    Anchor { start: u32, anchor_id: InTextBoxId },
}

pub(crate) fn parse_styled_text(
    text: &str,
    esc_char: char,
    start_block: char,
    end_block: char,
) -> crate::Result<ParsedStyledText> {
    let mut stack: Vec<StackEntry> = Vec::new();
    let mut result_text = String::with_capacity(text.len());
    let mut result_styles = Vec::new();
    let mut result_anchors = Vec::new();

    let mut input = text;

    let esc_len = esc_char.len_utf8();
    let start_len = start_block.len_utf8();
    let end_len = end_block.len_utf8();

    while !input.is_empty() {
        let mut esc_index = input.find(esc_char);
        let mut end_index = if stack.is_empty() {
            None
        } else {
            input.find(end_block)
        };
        if let (Some(idx1), Some(idx2)) = (esc_index, end_index) {
            if idx2 > idx1 {
                end_index = None;
            } else {
                esc_index = None;
            }
        }

        if let Some(idx) = esc_index {
            if idx > 0 {
                result_text.push_str(&input[..idx]);
            }
            let start = idx + esc_len;
            let end = input[start..].find(start_block).ok_or_else(|| {
                NelsieError::parsing_err(format!(
                    "Invalid style formatting: character '{esc_char}' found, but no following '{start_block}')"
                ))
            })? + start;
            let name = &input[start..end];
            if name.chars().all(|x| x.is_ascii_digit()) {
                stack.push(StackEntry::Anchor {
                    start: result_text.len() as u32,
                    anchor_id: name.parse().unwrap(),
                });
            } else {
                stack.push(StackEntry::Style {
                    start: result_text.len() as u32,
                    name,
                });
            }
            input = &input[end + start_len..];
        } else if let Some(idx) = end_index {
            if idx > 0 {
                result_text.push_str(&input[..idx]);
            }
            let end = result_text.len() as u32;
            match stack.pop().unwrap() {
                StackEntry::Style { start, name } => result_styles.push(ParsedStyleRange {
                    start,
                    end,
                    style: StyleOrName::Name(name),
                }),
                StackEntry::Anchor { start, anchor_id } => {
                    result_anchors.push((anchor_id, InTextAnchor { start, end }))
                }
            }
            input = &input[idx + end_len..];
        } else {
            result_text.push_str(input);
            break;
        };
    }

    if !stack.is_empty() {
        return Err(NelsieError::parsing_err("Unclosed style block"));
    }

    result_styles.reverse();
    result_styles.sort_by_key(|s: &ParsedStyleRange| (s.start, s.end));

    Ok(ParsedStyledText {
        text: result_text,
        styles: result_styles,
        anchors: result_anchors,
    })
}

#[cfg(test)]
mod tests {
    use crate::model::InTextAnchor as TA;
    use crate::parsers::text::{
        parse_styled_text, ParsedStyleRange, ParsedStyledText, StyleOrName,
    };

    fn parse(text: &str) -> crate::Result<ParsedStyledText> {
        parse_styled_text(text, '~', '{', '}')
    }

    #[test]
    fn test_parse_text_styles() {
        let r = parse("Hello").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(r.text, "Hello",);

        let r = parse("Hello\n Line 2 \n\n").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(r.text, "Hello\n Line 2 \n\n");

        let r = parse("xyz~name{ab}c").unwrap();
        assert_eq!(r.text, "xyzabc");
        assert_eq!(
            r.styles,
            vec![ParsedStyleRange {
                start: 3,
                end: 5,
                style: StyleOrName::Name("name")
            }]
        );

        let r = parse("~x{a}~y{b}~x{c}").unwrap();
        assert_eq!(
            r.styles,
            vec![
                ParsedStyleRange {
                    start: 0,
                    end: 1,
                    style: StyleOrName::Name("x")
                },
                ParsedStyleRange {
                    start: 1,
                    end: 2,
                    style: StyleOrName::Name("y")
                },
                ParsedStyleRange {
                    start: 2,
                    end: 3,
                    style: StyleOrName::Name("x")
                }
            ]
        );
        assert_eq!(r.text, "abc");

        let r = parse("~L1{~L2{~L3{x\n\nyy}}}").unwrap();
        assert_eq!(r.text, "x\n\nyy");
        assert_eq!(
            r.styles,
            vec![
                ParsedStyleRange {
                    start: 0,
                    end: 5,
                    style: StyleOrName::Name("L1")
                },
                ParsedStyleRange {
                    start: 0,
                    end: 5,
                    style: StyleOrName::Name("L2")
                },
                ParsedStyleRange {
                    start: 0,
                    end: 5,
                    style: StyleOrName::Name("L3")
                }
            ]
        );

        let r =
            parse("Hello, my name is ~name{Alice}.\n~question{How are ~highlight{you}?}").unwrap();
        assert_eq!(r.text, "Hello, my name is Alice.\nHow are you?");
        assert_eq!(
            r.styles,
            vec![
                ParsedStyleRange {
                    start: 18,
                    end: 23,
                    style: StyleOrName::Name("name")
                },
                ParsedStyleRange {
                    start: 25,
                    end: 37,
                    style: StyleOrName::Name("question")
                },
                ParsedStyleRange {
                    start: 33,
                    end: 36,
                    style: StyleOrName::Name("highlight")
                }
            ]
        );
    }

    #[test]
    fn test_parse_text_anchors() {
        let r = parse("abc~1{IJK}xyz").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(r.text, "abcIJKxyz");
        assert_eq!(r.anchors, vec![(1, TA { start: 3, end: 6 })]);

        let r = parse("~1{IJK}").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(r.text, "IJK");
        assert_eq!(r.anchors, vec![(1, TA { start: 0, end: 3 })]);

        let r = parse("~2{abc}~1{xy}").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(r.text, "abcxy");
        assert_eq!(
            r.anchors,
            vec![(2, TA { start: 0, end: 3 }), (1, TA { start: 3, end: 5 })]
        );

        let r = parse("a~name{b~1{c}d}e").unwrap();
        assert_eq!(r.text, "abcde");
        assert_eq!(r.anchors, vec![(1, TA { start: 2, end: 3 })]);
        assert_eq!(
            r.styles,
            vec![ParsedStyleRange {
                start: 1,
                end: 4,
                style: StyleOrName::Name("name")
            },]
        );

        let r = parse("a~21{~name{xxx}z}e").unwrap();
        assert_eq!(r.text, "axxxze");
        assert_eq!(r.anchors, vec![(21, TA { start: 1, end: 5 })]);
        assert_eq!(
            r.styles,
            vec![ParsedStyleRange {
                start: 1,
                end: 4,
                style: StyleOrName::Name("name")
            },]
        );

        let r = parse("~123{}").unwrap();
        assert_eq!(r.text, "");
        assert_eq!(r.anchors, vec![(123, TA { start: 0, end: 0 })]);
        assert!(r.styles.is_empty());

        let r = parse("ab~0{x\ny}z").unwrap();
        assert_eq!(r.text, "abx\nyz");
        assert_eq!(r.anchors, vec![(0, TA { start: 2, end: 5 })]);
        assert!(r.styles.is_empty());
    }
}
