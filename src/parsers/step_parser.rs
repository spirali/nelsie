use crate::model::{Step, StepValue};
use std::collections::BTreeMap;

pub(crate) fn parse_steps_from_label(value: &str) -> Option<(StepValue<bool>, Step)> {
    value.rsplit_once("**").and_then(|(_, b)| parse_steps(b))
}

pub(crate) fn parse_steps(value: &str) -> Option<(StepValue<bool>, Step)> {
    let mut value = value.trim_end();

    let mut until_end = false;
    if value.ends_with('+') {
        value = &value[0..value.len() - 1];
        until_end = true;
    }
    let mut steps = Vec::new();
    for part in value.split(',') {
        if let Some((part1, part2)) = part.split_once('-') {
            if let (Ok(step1), Ok(step2)) =
                (part1.trim().parse::<Step>(), part2.trim().parse::<Step>())
            {
                for step in step1..=step2 {
                    steps.push(step);
                }
            } else {
                return None;
            }
        } else if let Ok(step) = part.trim().parse::<Step>() {
            steps.push(step);
        } else {
            return None;
        }
    }

    let n_steps = steps.iter().max().copied().unwrap_or(0);
    let mut result = BTreeMap::new();
    result.insert(1, false);
    for step in &steps {
        if !steps.contains(&(step - 1)) {
            result.insert(*step, true);
        }
        if !steps.contains(&(step + 1)) {
            result.insert(*step + 1, false);
        }
    }
    if until_end {
        if let Some(m) = result.iter().next_back().map(|(k, _v)| *k) {
            result.remove(&m);
        }
    }
    Some((StepValue::from_btree(result), n_steps))
}

#[cfg(test)]
mod test {
    use crate::parsers::step_parser::parse_steps;

    #[test]
    pub fn test_parse() {
        assert_eq!(
            parse_steps("3").unwrap().0.key_values().collect::<Vec<_>>(),
            vec![(&1, &false), (&3, &true), (&4, &false)]
        );
        assert_eq!(
            parse_steps("1, 3")
                .unwrap()
                .0
                .key_values()
                .collect::<Vec<_>>(),
            vec![(&1, &true), (&2, &false), (&3, &true), (&4, &false)]
        );

        assert_eq!(
            parse_steps("1,2,4")
                .unwrap()
                .0
                .key_values()
                .collect::<Vec<_>>(),
            vec![(&1, &true), (&3, &false), (&4, &true), (&5, &false)]
        );

        assert_eq!(
            parse_steps("2+")
                .unwrap()
                .0
                .key_values()
                .collect::<Vec<_>>(),
            vec![(&1, &false), (&2, &true)]
        );

        assert_eq!(
            parse_steps("1, 3+")
                .unwrap()
                .0
                .key_values()
                .collect::<Vec<_>>(),
            vec![(&1, &true), (&2, &false), (&3, &true)]
        );

        assert_eq!(
            parse_steps("2-7, 10+")
                .unwrap()
                .0
                .key_values()
                .collect::<Vec<_>>(),
            vec![(&1, &false), (&2, &true), (&8, &false), (&10, &true)]
        );
    }
}
