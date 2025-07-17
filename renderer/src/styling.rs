use crate::text::{InlineBoxId, Text, TextStyle};

struct StyledText {

}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StyleOrName<'a> {
    Name(&'a str),
    Style(TextStyle),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParsedStyleRange<'a> {
    pub start: u32,
    pub end: u32,
    pub style: StyleOrName<'a>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct InlineAnchor {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub(crate) struct ParsedStyledText<'a> {
    pub text: String,
    pub styles: Vec<ParsedStyleRange<'a>>,
    pub anchors: Vec<(InlineBoxId, InlineAnchor)>,
}


pub(crate) fn parse_styled_text_from_plain_text(text: &str) -> ParsedStyledText {
    ParsedStyledText {
        text: text.to_string(),
        styles: Vec::new(),
        anchors: Default::default(),
    }
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
                    "Invalid style formatting: character '{}' found, but no following '{}')",
                    esc_char, start_block
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

impl StyledText {
    pub fn new(text: &Text) -> crate::Result<Self> {
            let mut parsed = if let Some(styling) = &text.styling{
                parse_styled_text(s, esc_char, start_block, end_block)?
            } else {
                parse_styled_text_from_plain_text(s)
            };
            if let Some(language) = &syntax_language {
                let theme = syntax_theme
                    .as_ref()
                    .ok_or_else(|| PyValueError::new_err("Invalid syntax highlight theme"))?;
                run_syntax_highlighting(resources, &mut parsed, language, theme)?;
            }

            let styles = parsed
                .styles
                .into_iter()
                .map(|style| {
                    Ok((
                        style.start,
                        style.end,
                        match style.style {
                            StyleOrName::Name(name) =>
                                Cow::Borrowed(styles.get_style(name)?),
                            StyleOrName::Style(style) => Cow::Owned(StepValue::Const(style)),
                        }
                    ))
                })
                .collect::<crate::Result<Vec<_>>>()?;
            Ok((parsed.text, styles, parsed.anchors))
        }
        )?;

        let mut steps: BTreeSet<&Step> = parsed.steps().collect();
        steps.extend(
            parsed
                .values()
                .flat_map(|v| v.1.iter().flat_map(|s| s.2.steps())),
        );
        steps.extend(main_style.steps());

        Ok(if steps.is_empty() {
            let (text, styles, anchors) = parsed.get_const().unwrap();
            let main_style = main_style
                .clone()
                .get_const()
                .unwrap()
                .into_text_style()
                .unwrap();
            let styles = styles
                .into_iter()
                .map(|(start, end, s)| StyledRange {
                    start,
                    end,
                    style: s.into_owned().get_const().unwrap(),
                })
                .collect_vec();
            StepValue::Const(StyledText {
                text,
                main_style,
                styles,
                anchors,
                text_align,
            })
        } else {
    let mut map = BTreeMap::new();
    for step in steps {
    let (text, styles, anchors) = parsed.at_step(step);
    let main_style = main_style.at_step(step).clone().into_text_style().unwrap();
    let styles = styles
    .iter()
    .map( |(start, end, s) | StyledRange {
    start: * start,
    end: * end,
    style: s.at_step(step).clone(),
    })
    .collect_vec();
    map.insert(
    step.clone(),
    StyledText {
    text: text.clone(),
    main_style: main_style.clone(),
    styles,
    anchors: anchors.clone(),
    text_align,
    },
    );
    }

}
