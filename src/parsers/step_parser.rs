use crate::model::{Step, StepSet, StepValue};
use crate::parsers::parse_utils::{parse_u32, CharParser, ParseError};
use chumsky::prelude::just;
use chumsky::text::TextParser;
use chumsky::Parser;
use std::collections::Bound::Unbounded;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::Excluded;

fn p_step() -> impl CharParser<Step> {
    parse_u32()
        .separated_by(just('.'))
        .at_least(1)
        .map(|v| Step::from_slice(v.as_slice()))
}

fn p_step_ignore_check() -> impl CharParser<(Step, bool)> {
    p_step().then(just('?').or_not().map(|v| v.is_none()))
}

enum StepDef {
    Single {
        step: Step,
        named: bool,
        exact: bool,
    },
    Unbounded {
        step: Step,
        named: bool,
    },
    Range {
        start: Step,
        start_named: bool,
        end: Step,
        end_named: bool,
        exact: bool,
    },
}

fn p_step_def() -> impl CharParser<StepDef> {
    just('!')
        .or_not()
        .map(|v| v.is_some())
        .then(p_step_ignore_check())
        .then(
            just('+')
                .padded()
                .map(|_| None)
                .or(just('-')
                    .padded()
                    .ignore_then(
                        just('!')
                            .or_not()
                            .map(|v| v.is_some())
                            .then(p_step_ignore_check()),
                    )
                    .map(Option::Some))
                .or_not(),
        )
        .validate(
            |((exact1, (step1, step1_named)), part2), span, emit| match part2 {
                Some(Some((exact2, (step2, step2_named)))) => {
                    if step1 > step2 && !step1.share_prefix(&step2) {
                        emit(ParseError::custom(span, "Range is not ordered"));
                    }
                    StepDef::Range {
                        start: step1,
                        start_named: step1_named,
                        end: step2,
                        end_named: step2_named,
                        exact: exact2,
                    }
                }
                Some(None) => StepDef::Unbounded {
                    step: step1,
                    named: step1_named,
                },
                _ => StepDef::Single {
                    step: step1,
                    named: step1_named,
                    exact: exact1,
                },
            },
        )
}

fn insert_step_range_into_map(map: &mut BTreeMap<Step, bool>, start: Step, end: Option<Step>) {
    if let Some(end) = end.as_ref() {
        if !map
            .range((Unbounded, Excluded(end)))
            .next_back()
            .map(|x| *x.1)
            .unwrap_or(false)
        {
            if map.contains_key(end) {
                map.remove(end);
            } else {
                map.insert(end.clone(), false);
            }
        }
        map.retain(|s, _| s < &start || s >= end);
    } else {
        map.retain(|s, _| s < &start);
    }

    if !map
        .range((Unbounded, Excluded(&start)))
        .next_back()
        .map(|x| *x.1)
        .unwrap_or(false)
    {
        map.insert(start, true);
    }
}

pub fn parse_step_flags() -> impl CharParser<(StepValue<bool>, BTreeSet<Step>)> {
    p_step_def().separated_by(just(',').padded()).map(|defs| {
        let mut btree = BTreeMap::new();
        let mut named_steps = BTreeSet::new();
        for def in defs {
            match def {
                StepDef::Single { step, named, exact } => {
                    if named {
                        named_steps.insert(step.clone());
                    }
                    let end = if exact {
                        step.first_substep()
                    } else {
                        step.next()
                    };
                    insert_step_range_into_map(&mut btree, step, Some(end));
                }
                StepDef::Range {
                    start,
                    start_named,
                    end,
                    end_named,
                    exact,
                } => {
                    if start_named {
                        named_steps.insert(start.clone());
                    }
                    if end_named {
                        named_steps.insert(end.clone());
                    }
                    let end = if exact {
                        end.first_substep()
                    } else {
                        end.next()
                    };
                    insert_step_range_into_map(&mut btree, start, Some(end));
                }
                StepDef::Unbounded { named, step } => {
                    if named {
                        named_steps.insert(step.clone());
                    }
                    insert_step_range_into_map(&mut btree, step, None);
                }
            }
        }
        (StepValue::new_map(btree), named_steps)
    })
}

pub(crate) fn parse_steps_from_label(
    value: &str,
    steps: Option<&mut StepSet>,
) -> Option<StepValue<bool>> {
    value
        .rsplit_once("**")
        .and_then(|(_, b)| parse_steps(b, steps))
}

pub(crate) fn parse_steps(value: &str, steps: Option<&mut StepSet>) -> Option<StepValue<bool>> {
    // TODO: Better error messages.
    // I do not know why, chumsky now returns on every error something like "unexpected end of input"
    // So we are actually not using any information by returning just None
    let (step_value, mut named_steps) = parse_step_flags().parse_text(value).ok()?;
    if let Some(s) = steps {
        s.append(&mut named_steps);
    };
    Some(step_value)
}

#[cfg(test)]
mod test {
    use crate::model::Step;
    use crate::parsers::parse_utils::CharParser;
    use crate::parsers::step_parser::{p_step, p_step_ignore_check, parse_step_flags};
    use itertools::Itertools;

    #[test]
    pub fn test_step_parse() {
        assert_eq!(p_step().parse_text("0").unwrap(), Step::from_int(0));
        assert_eq!(p_step().parse_text("1").unwrap(), Step::from_int(1));
        assert_eq!(p_step().parse_text("10").unwrap(), Step::from_int(10));
        assert_eq!(
            p_step().parse_text("10.20.30").unwrap(),
            Step::from_slice(&[10, 20, 30])
        );
        assert!(p_step().parse_text("9999999999").is_err());
        assert!(p_step().parse_text("").is_err());
    }

    #[test]
    pub fn test_step_parse_named_check() {
        assert_eq!(
            p_step_ignore_check().parse_text("123").unwrap(),
            (Step::from_int(123), true)
        );
        assert_eq!(
            p_step_ignore_check().parse_text("123?").unwrap(),
            (Step::from_int(123), false)
        );
        assert_eq!(
            p_step_ignore_check().parse_text("10.20").unwrap(),
            (Step::from_slice(&[10, 20]), true)
        );
        assert_eq!(
            p_step_ignore_check().parse_text("9.1.2?").unwrap(),
            (Step::from_slice(&[9, 1, 2]), false)
        );
    }

    fn _parse_steps(input: &str, vals: &[(Step, bool)], named: &[Step]) {
        let (values, names_steps) = parse_step_flags().parse_text(input).unwrap();
        assert_eq!(
            values
                .key_values()
                .filter(|(k, v)| *v || *k != Step::from_int(0))
                .collect_vec(),
            vals
        );
        assert_eq!(names_steps.into_iter().collect_vec(), named);
    }

    #[test]
    pub fn test_parse_step_bool() {
        let s = Step::from_int;
        let sv = Step::from_slice;

        _parse_steps("5", &[(s(5), true), (s(6), false)], &[s(5)]);
        _parse_steps("5,5,5", &[(s(5), true), (s(6), false)], &[s(5)]);
        _parse_steps("5, 6", &[(s(5), true), (s(7), false)], &[s(5), s(6)]);
        _parse_steps("6, 5", &[(s(5), true), (s(7), false)], &[s(5), s(6)]);
        _parse_steps(
            "5, 7",
            &[(s(5), true), (s(6), false), (s(7), true), (s(8), false)],
            &[s(5), s(7)],
        );
        _parse_steps(
            "7, 5",
            &[(s(5), true), (s(6), false), (s(7), true), (s(8), false)],
            &[s(5), s(7)],
        );

        _parse_steps(
            "10-20, 15, 10, 20",
            &[(s(10), true), (s(21), false)],
            &[s(10), s(15), s(20)],
        );
        _parse_steps(
            "10, 20, 15,10-20",
            &[(s(10), true), (s(21), false)],
            &[s(10), s(15), s(20)],
        );
        _parse_steps(
            "2, 3-4, 10, 12-14",
            &[
                (s(2), true),
                (s(5), false),
                (s(10), true),
                (s(11), false),
                (s(12), true),
                (s(15), false),
            ],
            &[s(2), s(3), s(4), s(10), s(12), s(14)],
        );
        _parse_steps("1-1", &[(s(1), true), (s(2), false)], &[s(1)]);
        _parse_steps(
            "3.5-3.7",
            &[(sv(&[3, 5]), true), (sv(&[3, 8]), false)],
            &[sv(&[3, 5]), sv(&[3, 7])],
        );
        _parse_steps(
            "1.5-1",
            &[(sv(&[1, 5]), true), (sv(&[2]), false)],
            &[sv(&[1]), sv(&[1, 5])],
        );
        _parse_steps(
            "3,4?,5,6?-7?",
            &[(s(3), true), (s(8), false)],
            &[s(3), s(5)],
        );

        _parse_steps(
            "!4.1",
            &[(sv(&[4, 1]), true), (sv(&[4, 1, 0]), false)],
            &[sv(&[4, 1])],
        );

        _parse_steps(
            "2-!2.5",
            &[(sv(&[2]), true), (sv(&[2, 5, 0]), false)],
            &[s(2), sv(&[2, 5])],
        );

        _parse_steps("123+", &[(s(123), true)], &[s(123)]);
        _parse_steps("123?+", &[(s(123), true)], &[]);

        _parse_steps("2+, 4, 10?-12?", &[(s(2), true)], &[s(2), s(4)]);
        _parse_steps("4, 10?-12?, 2+", &[(s(2), true)], &[s(2), s(4)]);

        _parse_steps("3-7, 4+", &[(s(3), true)], &[s(3), s(4), s(7)]);
        _parse_steps("4+, 3-7", &[(s(3), true)], &[s(3), s(4), s(7)]);
    }

    #[test]
    pub fn test_parse_step_bool_errors() {
        assert!(parse_step_flags().parse_text("3-1").is_err());
        assert!(parse_step_flags().parse_text("2.5-1").is_err());
    }
}
