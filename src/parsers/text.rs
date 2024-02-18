use crate::common::error::NelsieError;
use crate::model::{
    InTextAnchor, InTextAnchorId, InTextAnchorPoint, PartialTextStyle, Span, StyledLine,
};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StyleOrName<'a> {
    Name(&'a str),
    Style(PartialTextStyle),
}

#[derive(Debug)]
pub(crate) struct ParsedStyledText<'a> {
    pub styled_lines: Vec<StyledLine>,
    pub styles: Vec<Vec<StyleOrName<'a>>>,
    pub anchors: HashMap<InTextAnchorId, InTextAnchor>,
}

fn find_first(text: &str, c1: char, c2: Option<char>) -> Option<(usize, char)> {
    for (i, c) in text.char_indices() {
        if c1 == c || c2.map(|c2| c2 == c).unwrap_or(false) {
            return Some((i, c));
        }
    }
    None
}

pub(crate) fn parse_styled_text_from_plain_text(text: &str) -> ParsedStyledText {
    ParsedStyledText {
        styled_lines: text
            .lines()
            .map(|line| StyledLine {
                spans: vec![Span {
                    length: line.len() as u32,
                    style_idx: 0,
                }],
                text: line.to_string(),
            })
            .collect(),
        styles: vec![vec![]],
        anchors: Default::default(),
    }
}

pub(crate) fn parse_styled_text<'a>(
    text: &'a str,
    esc_char: char,
    start_block: char,
    end_block: char,
) -> crate::Result<ParsedStyledText> {
    let mut style_stack: Vec<StyleOrName<'a>> = Vec::new();
    let mut anchor_stack: Vec<Option<(InTextAnchorId, InTextAnchorPoint)>> = Vec::new();

    let mut result_styles = Vec::new();
    let mut result_anchors = HashMap::<InTextAnchorId, InTextAnchor>::new();

    let get_style = |stack: &[StyleOrName<'a>], styles: &mut Vec<Vec<StyleOrName<'a>>>| {
        styles.iter().position(|s| s == stack).unwrap_or_else(|| {
            let idx = styles.len();
            styles.push(stack.to_vec());
            idx
        })
    };

    let mut out_lines: Vec<StyledLine> = Vec::new();
    for (line_idx, mut line) in text.lines().enumerate() {
        let mut result_text = String::new();
        let mut spans = Vec::new();
        loop {
            if let Some((idx, c)) = find_first(
                line,
                esc_char,
                if anchor_stack.is_empty() {
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
                    let idx = line.find(start_block).ok_or_else(|| {
                        NelsieError::parsing_err(format!(
                            "Invalid style formatting (line {}): character '{}' found, but no following '{}')",
                            line_idx + 1, esc_char, start_block
                        ))
                    })?;
                    let style_name = &line[..idx];
                    if style_name.chars().all(|x| x.is_ascii_digit()) {
                        let anchor_id: InTextAnchorId = style_name
                            .parse()
                            .map_err(|_| NelsieError::parsing_err("Invalid anchor id"))?;
                        anchor_stack.push(Some((
                            anchor_id,
                            InTextAnchorPoint {
                                line_idx: line_idx as u32,
                                span_idx: spans.len() as u32,
                            },
                        )));
                    } else {
                        anchor_stack.push(None);
                        style_stack.push(StyleOrName::Name(style_name));
                    }
                    line = &line[idx + 1..];
                } else if let Some((anchor_id, start)) = anchor_stack.pop().unwrap() {
                    result_anchors.insert(
                        anchor_id,
                        InTextAnchor {
                            start,
                            end: InTextAnchorPoint {
                                line_idx: line_idx as u32,
                                span_idx: spans.len() as u32,
                            },
                        },
                    );
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
        anchors: result_anchors,
    })
}

#[cfg(test)]
mod tests {
    use crate::model::{InTextAnchor as TA, InTextAnchorPoint as TAP, Span, StyledLine};
    use crate::parsers::text::{parse_styled_text, ParsedStyledText, StyleOrName};

    fn parse(text: &str) -> crate::Result<ParsedStyledText> {
        parse_styled_text(text, '~', '{', '}')
    }

    #[test]
    fn test_parse_text_styles() {
        let r = parse("Hello").unwrap();
        assert_eq!(r.styles, vec![Vec::<_>::new()]);
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
        assert_eq!(r.styles, vec![Vec::<_>::new()]);
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
        assert_eq!(r.styles, vec![vec![], vec![StyleOrName::Name("name")]]);
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
        assert_eq!(
            r.styles,
            vec![vec![StyleOrName::Name("x")], vec![StyleOrName::Name("y")]]
        );
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
        assert_eq!(
            r.styles,
            vec![vec![
                StyleOrName::Name("L1"),
                StyleOrName::Name("L2"),
                StyleOrName::Name("L3")
            ]]
        );
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
                vec![StyleOrName::Name("name")],
                vec![StyleOrName::Name("question")],
                vec![
                    StyleOrName::Name("question"),
                    StyleOrName::Name("highlight")
                ]
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

    #[test]
    fn test_parse_text_anchors() {
        let r = parse("abc~1{IJK}xyz").unwrap();
        assert_eq!(r.styles, vec![Vec::<_>::new()]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![
                    Span {
                        length: 3,
                        style_idx: 0
                    },
                    Span {
                        length: 3,
                        style_idx: 0
                    },
                    Span {
                        length: 3,
                        style_idx: 0
                    }
                ],
                text: "abcIJKxyz".to_string(),
            }]
        );
        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&1).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 2
                },
            }
        );

        let r = parse("~1{IJK}").unwrap();
        assert_eq!(r.styles, vec![Vec::<_>::new()]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![Span {
                    length: 3,
                    style_idx: 0
                }],
                text: "IJK".to_string(),
            }]
        );
        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&1).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 0
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
            }
        );

        let r = parse("~2{abc}~1{xy}").unwrap();
        assert_eq!(r.styles, vec![Vec::<_>::new()]);
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
                        style_idx: 0
                    }
                ],
                text: "abcxy".to_string(),
            }]
        );
        assert_eq!(r.anchors.len(), 2);
        assert_eq!(
            r.anchors.get(&2).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 0
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
            }
        );
        assert_eq!(
            r.anchors.get(&1).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 2
                },
            }
        );

        let r = parse("a~name{b~1{c}d}e").unwrap();
        assert_eq!(r.styles, vec![vec![], vec![StyleOrName::Name("name")]]);
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
                        style_idx: 1
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
                text: "abcde".to_string(),
            }]
        );
        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&1).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 2
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 3
                },
            }
        );

        let r = parse("a~21{~name{xxx}z}e").unwrap();
        assert_eq!(r.styles, vec![vec![], vec![StyleOrName::Name("name")]]);
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![
                    Span {
                        length: 1,
                        style_idx: 0
                    },
                    Span {
                        length: 3,
                        style_idx: 1
                    },
                    Span {
                        length: 1,
                        style_idx: 0
                    },
                    Span {
                        length: 1,
                        style_idx: 0
                    },
                ],
                text: "axxxze".to_string(),
            }]
        );

        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&21).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 3
                },
            }
        );

        let r = parse("~123{}").unwrap();
        assert!(r.styles.is_empty());
        assert_eq!(
            r.styled_lines,
            vec![StyledLine {
                spans: vec![],
                text: "".to_string(),
            }]
        );

        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&123).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 0
                },
                end: TAP {
                    line_idx: 0,
                    span_idx: 0
                },
            }
        );

        let r = parse("ab~0{x\ny}z").unwrap();
        assert_eq!(r.styles.len(), 1);
        assert_eq!(
            r.styled_lines,
            vec![
                StyledLine {
                    spans: vec![
                        Span {
                            length: 2,
                            style_idx: 0
                        },
                        Span {
                            length: 1,
                            style_idx: 0
                        }
                    ],
                    text: "abx".to_string(),
                },
                StyledLine {
                    spans: vec![
                        Span {
                            length: 1,
                            style_idx: 0
                        },
                        Span {
                            length: 1,
                            style_idx: 0
                        }
                    ],
                    text: "yz".to_string(),
                }
            ]
        );

        assert_eq!(r.anchors.len(), 1);
        assert_eq!(
            r.anchors.get(&0).unwrap(),
            &TA {
                start: TAP {
                    line_idx: 0,
                    span_idx: 1
                },
                end: TAP {
                    line_idx: 1,
                    span_idx: 1
                },
            }
        );
    }
}
