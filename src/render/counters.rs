use crate::model::{SlideDeck, SlideId, Step, StyledText};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

pub(crate) type CountersMap<'a> = HashMap<&'a str, Counter>;

#[derive(Debug)]
pub(crate) struct Indices {
    pub(crate) slide_idx: u32,
    pub(crate) page_idx: u32,
}

#[derive(Debug)]
pub(crate) struct Counter {
    pub(crate) indices: BTreeMap<(SlideId, Step), Indices>,
    pub(crate) n_slides: u32,
    pub(crate) n_pages: u32,
}

impl Counter {
    fn new(page_ordering: &[(bool, u32, Step)]) -> Counter {
        let mut indices = BTreeMap::new();
        let mut slide_idx = 0;
        let mut page_idx = 0;
        let mut prev_slide_id = u32::MAX;
        for (active, slide_id, step) in page_ordering.iter() {
            if *active {
                page_idx += 1;
                if *slide_id != prev_slide_id {
                    slide_idx += 1;
                    prev_slide_id = *slide_id;
                }
            }
            indices.insert(
                (*slide_id, step.clone()),
                Indices {
                    slide_idx: if slide_idx == 0 { 0 } else { slide_idx - 1 },
                    page_idx: if page_idx == 0 { 0 } else { page_idx - 1 },
                },
            );
        }
        Counter {
            n_pages: page_idx,
            n_slides: slide_idx,
            indices,
        }
    }
}

pub(crate) fn replace_counters(
    counter_values: &CountersMap,
    styled_text: &mut StyledText,
    slide_id: u32,
    step: &Step,
) {
    for (name, values) in counter_values {
        let Indices {
            slide_idx,
            page_idx,
        } = values.indices.get(&(slide_id, step.clone())).unwrap();
        styled_text.replace_text(&format!("$({name}_slide)"), &(slide_idx + 1).to_string());
        styled_text.replace_text(&format!("$({name}_page)"), &(page_idx + 1).to_string());
        styled_text.replace_text(&format!("$({name}_slides)"), &values.n_slides.to_string());
        styled_text.replace_text(&format!("$({name}_pages)"), &values.n_pages.to_string());
    }
}

pub(crate) fn compute_counters(slide_deck: &SlideDeck) -> CountersMap {
    let mut global_pages: Vec<(bool, SlideId, Step)> = Vec::new();
    let mut counter_names: HashSet<&String> = HashSet::new();
    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        let slide_idx = slide_idx as u32;
        for name in &slide.counters {
            counter_names.insert(name);
        }
        let after_last = slide
            .visible_steps()
            .last()
            .map(|s| s.first_substep())
            .unwrap_or_default();

        if let Some((parent_idx, parent_step)) = &slide.parent {
            let pos = global_pages
                .iter()
                .enumerate()
                .filter(|(_, (_, id, step))| id == parent_idx && step <= parent_step)
                .max_by_key(|(_, (_, _, step))| step)
                .unwrap()
                .0;
            for (i, step) in slide.visible_steps().enumerate() {
                global_pages.insert(pos + i, (true, slide_idx, step.clone()))
            }
            global_pages.insert(
                pos + slide.visible_steps().count(),
                (false, slide_idx, after_last),
            )
        } else {
            for step in slide.visible_steps() {
                global_pages.push((true, slide_idx, step.clone()))
            }
            global_pages.push((false, slide_idx, after_last))
        }
    }

    // Remove dummy markers
    global_pages.retain(|(active, _, _)| *active);

    let global_counter = Counter::new(&global_pages);
    let mut map = CountersMap::new();
    for name in counter_names {
        let pages = global_pages
            .iter()
            .map(|(_, slide_idx, step)| {
                (
                    slide_deck.slides[*slide_idx as usize]
                        .counters
                        .contains(name),
                    *slide_idx,
                    step.clone(),
                )
            })
            .collect_vec();
        map.insert(name, Counter::new(&pages));
    }
    map.insert("global", global_counter);
    map
}
