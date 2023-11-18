use crate::model::{Step, StepValue};

pub(crate) fn parse_steps_from_label(value: &str) -> Option<(StepValue<bool>, Step)> {
    value.rsplit_once("**").and_then(|(_, b)| parse_steps(b))
}

fn parse_steps(value: &str) -> Option<(StepValue<bool>, Step)> {
    let mut value = value.trim_end();

    let mut until_end = false;
    if value.ends_with("+") {
        value = &value[0..value.len() - 1];
        until_end = true;
    }
    let mut steps = Vec::new();
    for part in value.split(",") {
        if let Some((part1, part2)) = part.split_once("-") {
            if let (Ok(step1), Ok(step2)) =
                (part1.trim().parse::<Step>(), part2.trim().parse::<Step>())
            {
                for step in step1..=step2 {
                    steps.push(step);
                }
            } else {
                return None;
            }
        } else {
            if let Ok(step) = part.trim().parse::<Step>() {
                steps.push(step);
            } else {
                return None;
            }
        }
    }

    let n_steps = steps.iter().max().copied().unwrap_or(0);
    let m_steps = if until_end { n_steps } else { n_steps + 1 };
    let mut result = vec![false; m_steps as usize];
    for step in steps {
        if step > 0 {
            result[(step - 1) as usize] = true;
        }
    }
    Some((StepValue::from_vec(result), n_steps))
}

#[cfg(test)]
mod test {
    use crate::common::step_parser::parse_steps;

    #[test]
    pub fn test_parse() {
        assert_eq!(
            parse_steps("3")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![false, false, true, false]
        );
        assert_eq!(
            parse_steps("1,3")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![true, false, true, false]
        );
        assert_eq!(
            parse_steps("1,2,4")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![true, true, false, true, false]
        );
        assert_eq!(
            parse_steps("2+")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![false, true]
        );
        assert_eq!(
            parse_steps("1, 3+")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![true, false, true]
        );
        assert_eq!(
            parse_steps("2-7, 10+")
                .unwrap().0
                .values()
                .copied()
                .collect::<Vec<_>>(),
            vec![false, true, true, true, true, true, true, false, false, true]
        );
    }
}
