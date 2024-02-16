use crate::common::Step;
use crate::model::{SlideDeck, StyledText};
use std::collections::Bound::Unbounded;
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::Excluded;

#[derive(Default)]
pub(crate) struct SlideCounterValue {
    pub(crate) slide_idx: u32,
    pub(crate) page_idx: u32,
}

#[derive(Default)]
pub(crate) struct CounterValues {
    pub(crate) per_slide: BTreeMap<usize, SlideCounterValue>,
    pub(crate) n_slides: u32,
    pub(crate) n_pages: u32,
}

pub(crate) type CountersMap<'a> = HashMap<&'a str, CounterValues>;

pub(crate) fn replace_counters(
    counter_values: &CountersMap,
    styled_text: &mut StyledText,
    slide_idx: usize,
    step: Step,
) {
    for (name, values) in counter_values {
        let (slide_idx, page_idx) = values
            .per_slide
            .get(&slide_idx)
            .map(|x| (x.slide_idx, x.page_idx + step))
            .unwrap_or_else(|| {
                values
                    .per_slide
                    .range((Excluded(slide_idx), Unbounded))
                    .next()
                    .map(|x| (x.1.slide_idx - 1, x.1.page_idx))
                    .unwrap_or((values.n_slides, values.n_pages))
            });
        styled_text.replace_text(&format!("$({name}_slide)"), &slide_idx.to_string());
        styled_text.replace_text(&format!("$({name}_page)"), &page_idx.to_string());
        styled_text.replace_text(&format!("$({name}_slides)"), &values.n_slides.to_string());
        styled_text.replace_text(&format!("$({name}_pages)"), &values.n_pages.to_string());
    }
}

pub(crate) fn compute_counters(slide_deck: &SlideDeck) -> CountersMap {
    let mut counter_values = CountersMap::new();
    let mut global_counter = CounterValues {
        per_slide: Default::default(),
        n_slides: slide_deck.slides.len() as u32,
        n_pages: 0,
    };

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        global_counter.per_slide.insert(
            slide_idx,
            SlideCounterValue {
                slide_idx: slide_idx as u32 + 1,
                page_idx: global_counter.n_pages,
            },
        );
        global_counter.n_pages += slide.n_steps;
        for counter_name in &slide.counters {
            let v = counter_values.entry(counter_name).or_default();
            v.n_slides += 1;
            v.per_slide.insert(
                slide_idx,
                SlideCounterValue {
                    slide_idx: v.n_slides,
                    page_idx: v.n_pages,
                },
            );
            v.n_pages += slide.n_steps;
        }
    }
    counter_values.insert("global", global_counter);
    counter_values
}
