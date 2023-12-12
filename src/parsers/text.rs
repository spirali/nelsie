use crate::common::error::NelsieError;
use crate::model::{Span, StyledLine};

#[derive(Debug)]
pub(crate) struct ParsedStyledText<'a> {
    pub styled_lines: Vec<StyledLine>,
    pub styles: Vec<Vec<&'a str>>,
}

fn find_first(text: &str, c1: char, c2: Option<char>) -> Option<(usize, char)> {
    for (i, c) in text.char_indices() {
        if c1 == c || c2.map(|c2| c2 == c).unwrap_or(false) {
            return Some((i, c));
        }
    }
    None
}

pub(crate) fn parse_styled_text<'a>(
    text: &'a str,
    esc_char: char,
    start_block: char,
    end_block: char,
) -> crate::Result<ParsedStyledText> {
    let mut style_stack: Vec<&str> = Vec::new();
    let mut result_styles = Vec::new();

    let get_style = |stack: &[&'a str], styles: &mut Vec<Vec<&'a str>>| {
        styles.iter().position(|s| s == stack).unwrap_or_else(|| {
            let idx = styles.len();
            styles.push(stack.to_vec());
            idx
        })
    };

    let mut out_lines: Vec<StyledLine> = Vec::new();
    for mut line in text.lines() {
        let mut result_text = String::new();
        let mut spans = Vec::new();
        loop {
            if let Some((idx, c)) = find_first(
                line,
                esc_char,
                if style_stack.is_empty() {
                    None
                } else {
                    Some(end_block)
                },
            ) {
                if idx > 0 {
                    result_text.push_str(&line[..idx]);
                    spans.push(Span {
                        length: idx as u32,
                        style_idx: get_style(&style_stack, &mut result_styles) as u32,
                    });
                }
                line = &line[idx + 1..];
                if c == esc_char {
                    let idx = line
                        .find(start_block)
                        .ok_or_else(|| NelsieError::parsing_err("Invalid style formatting"))?;
                    style_stack.push(&line[..idx]);
                    line = &line[idx + 1..];
                } else {
                    style_stack.pop();
                }
            } else {
                if !line.is_empty() {
                    spans.push(Span {
                        length: line.len() as u32,
                        style_idx: get_style(&style_stack, &mut result_styles) as u32,
                    });
                    result_text.push_str(line);
                }
                out_lines.push(StyledLine {
                    spans,
                    text: result_text,
                });
                break;
            }
        }
    }
    Ok(ParsedStyledText {
        styled_lines: out_lines,
        styles: result_styles,
    })
}

#[cfg(test)]
mod tests {
    use crate::model::{Span, StyledLine};
    use crate::parsers::text::{parse_styled_text, ParsedStyledText};

    fn parse(text: &str) -> crate::Result<ParsedStyledText> {
        parse_styled_text(text, '~', '{', '}')
    }

    #[test]
    fn test_parse_text() {
        let r = parse("Hello").unwrap();
        assert_eq!(r.styles, vec![Vec::<String>::new()]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![Span {
                    length: 5,
                    style_idx: 0
                }],
                text: "Hello".to_string(),
            }]
        );

        let r = parse("Hello\n Line 2 \n\n").unwrap();
        assert_eq!(r.styles, vec![Vec::<String>::new()]);
        assert_eq!(
            r.styled_lines,
            vec![
                StyledLine {
                    spans: vec![Span {
                        length: 5,
                        style_idx: 0
                    }],
                    text: "Hello".to_string(),
                },
                StyledLine {
                    spans: vec![Span {
                        length: 8,
                        style_idx: 0
                    }],
                    text: " Line 2 ".to_string(),
                },
                StyledLine {
                    spans: vec![],
                    text: "".to_string(),
                }
            ]
        );

        let r = parse("xyz~name{ab}c").unwrap();
        assert_eq!(r.styles, vec![vec![], vec!["name"]]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![
                    Span {
                        length: 3,
                        style_idx: 0
                    },
                    Span {
                        length: 2,
                        style_idx: 1
                    },
                    Span {
                        length: 1,
                        style_idx: 0
                    }
                ],
                text: "xyzabc".to_string(),
            }]
        );

        let r = parse("~x{a}~y{b}~x{c}").unwrap();
        assert_eq!(r.styles, vec![vec!["x"], vec!["y"]]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![
                    Span {
                        length: 1,
                        style_idx: 0
                    },
                    Span {
                        length: 1,
                        style_idx: 1
                    },
                    Span {
                        length: 1,
                        style_idx: 0
                    }
                ],
                text: "abc".to_string(),
            }]
        );

        let r = parse("~L1{~L2{~L3{x\n\nyy}}}").unwrap();
        assert_eq!(r.styles, vec![vec!["L1", "L2", "L3"]]);
        assert_eq!(
            r.styled_lines,
            vec![
                StyledLine {
                    spans: vec![Span {
                        length: 1,
                        style_idx: 0
                    },],
                    text: "x".to_string(),
                },
                StyledLine {
                    spans: vec![],
                    text: "".to_string(),
                },
                StyledLine {
                    spans: vec![Span {
                        length: 2,
                        style_idx: 0
                    },],
                    text: "yy".to_string(),
                }
            ]
        );

        let r =
            parse("Hello, my name is ~name{Alice}.\n~question{How are ~highlight{you}?").unwrap();
        assert_eq!(
            r.styles,
            vec![
                vec![],
                vec!["name"],
                vec!["question"],
                vec!["question", "highlight"]
            ]
        );
        assert_eq!(
            r.styled_lines,
            vec![
                StyledLine {
                    spans: vec![
                        Span {
                            length: 18,
                            style_idx: 0
                        },
                        Span {
                            length: 5,
                            style_idx: 1
                        },
                        Span {
                            length: 1,
                            style_idx: 0
                        }
                    ],
                    text: "Hello, my name is Alice.".to_string(),
                },
                StyledLine {
                    spans: vec![
                        Span {
                            length: 8,
                            style_idx: 2
                        },
                        Span {
                            length: 3,
                            style_idx: 3
                        },
                        Span {
                            length: 1,
                            style_idx: 2
                        }
                    ],
                    text: "How are you?".to_string()
                }
            ]
        );
    }
}
