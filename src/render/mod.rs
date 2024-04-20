mod canvas;
mod counters;
mod image;
mod layout;
mod pagebuilder;
mod pathbuilder;
mod paths;
mod pdf;
mod rendering;
mod text;

use crate::model::{FontData, Resources, Slide, SlideDeck, SlideId};
pub(crate) use pdf::PdfBuilder;

use crate::common::Step;
use crate::render::counters::{compute_counters, CountersMap};
use crate::render::pagebuilder::PageBuilder;
use crate::render::rendering::render_to_canvas;
use std::path::Path;
use std::sync::Arc;

pub(crate) struct RenderConfig<'a> {
    pub resources: &'a Resources,
    pub slide: &'a Slide,
    pub slide_id: SlideId,
    pub step: Step,
    pub default_font: &'a Arc<FontData>,
    pub counter_values: &'a CountersMap<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum OutputFormat {
    Pdf,
    Svg,
    Png,
}

pub(crate) struct OutputConfig<'a> {
    pub path: Option<&'a Path>,
    pub format: OutputFormat,
}

pub(crate) enum VerboseLevel {
    Silent,
    Normal,
    Full,
}

impl VerboseLevel {
    pub fn is_full(&self) -> bool {
        match self {
            VerboseLevel::Silent | VerboseLevel::Normal => false,
            VerboseLevel::Full => true,
        }
    }
    pub fn is_normal_or_more(&self) -> bool {
        match self {
            VerboseLevel::Silent => false,
            VerboseLevel::Normal | VerboseLevel::Full => true,
        }
    }
}

fn render_slide(
    resources: &Resources,
    builder: &PageBuilder,
    slide_id: SlideId,
    slide: &Slide,
    default_font: &Arc<FontData>,
    counter_values: &CountersMap,
) -> crate::Result<()> {
    log::debug!("Rendering slide {}", slide_id);
    for step in 1..=slide.n_steps {
        let render_cfg = RenderConfig {
            resources,
            slide,
            slide_id,
            step,
            default_font,
            counter_values,
        };
        let canvas = render_to_canvas(&render_cfg);
        let counter = render_cfg.counter_values.get("global").unwrap();
        let page_idx = counter
            .indices
            .get(&(render_cfg.slide_id, render_cfg.step))
            .unwrap()
            .page_idx;
        builder.add_page(slide_id, step, page_idx, canvas, render_cfg.resources)?
    }
    Ok(())
}

pub(crate) fn render_slide_deck(
    slide_deck: &SlideDeck,
    resources: &Resources,
    output_cfg: &OutputConfig,
    verbose_level: VerboseLevel,
) -> crate::Result<Vec<(usize, usize, Vec<u8>)>> {
    let start_time = std::time::Instant::now();
    if verbose_level.is_full() {
        println!(
            "Slides construction: {:.2}s",
            (start_time - slide_deck.creation_time).as_secs_f32()
        );
    }

    let counter_values = compute_counters(slide_deck);
    let global_counter = counter_values.get("global").unwrap();

    let progress_bar = if verbose_level.is_normal_or_more() {
        Some(indicatif::ProgressBar::new(global_counter.n_pages.into()))
    } else {
        None
    };
    let builder = PageBuilder::new(slide_deck, output_cfg, progress_bar, global_counter.n_pages)?;

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        render_slide(
            resources,
            &builder,
            slide_idx as SlideId,
            slide,
            &slide_deck.default_font,
            &counter_values,
        )?;
    }

    let result_data = builder.finish()?;

    if verbose_level.is_full() {
        let render_end_time = std::time::Instant::now();
        println!(
            "Total rendering time: {:.2}s",
            (render_end_time - start_time).as_secs_f32()
        );
    }

    Ok(result_data)
}
