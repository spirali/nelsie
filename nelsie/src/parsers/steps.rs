use crate::common::steps::Step;
use std::cmp::Ordering;

type BoolStepVal = (Vec<(Step, bool)>, Vec<Step>);

pub fn parse_step(input: &str) -> crate::Result<(Step, bool, bool)> {
    let (input, exact) = input
        .strip_prefix('!')
        .map(|s| (s, true))
        .unwrap_or((input, false));
    let (input, silent) = input
        .strip_suffix('?')
        .map(|s| (s, true))
        .unwrap_or((input, false));
    let indices = input
        .split(".")
        .map(|s| {
            let v: u32 = s
                .parse()
                .map_err(|_| crate::Error::Parsing("Invalid step definition".to_string()))?;
            Ok(v)
        })
        .collect::<crate::Result<_>>()?;
    Ok((Step::new(indices), exact, silent))
}

pub fn parse_bool_steps(input: &str) -> crate::Result<BoolStepVal> {
    let mut result = Vec::new();
    let mut named = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if let Some(part) = part.strip_suffix('+') {
            let (step, _, silent) = parse_step(part)?;
            if !silent {
                named.push(step.clone());
            }
            result.push((step, true));
        } else if let Some((part1, part2)) = part.split_once('-') {
            let part1 = part1.trim();
            let part2 = part2.trim();
            let (step1, _, silent1) = parse_step(part1)?;
            let (mut step2, exact, silent2) = parse_step(part2)?;
            if !silent1 {
                named.push(step1.clone());
            }
            if !silent2 {
                named.push(step2.clone());
            }
            result.push((step1, true));
            if exact {
                step2.first_substep();
            } else {
                step2.next();
            }
            result.push((step2, false));
        } else {
            let (step, exact, silent) = parse_step(part)?;
            if !silent {
                named.push(step.clone());
            }
            let mut end = step.clone();
            if exact {
                end.first_substep();
            } else {
                end.next();
            }
            result.push((step, true));
            result.push((end, false));
        }
    }
    result.sort_unstable_by(|(step1, onoff1), (step2, onoff2)| {
        let r = step1.cmp(step2);
        if r == Ordering::Equal {
            onoff1.cmp(onoff2).reverse()
        } else {
            r
        }
    });
    result.dedup_by(|(step1, _), (step2, _)| step1 == step2);
    result.dedup_by_key(|a| a.1);
    named.sort_unstable();
    Ok((result, named))
}
