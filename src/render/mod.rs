mod canvas;
mod canvas_pdf;
mod canvas_svg;
mod counters;
mod image;
mod layout;
mod pagebuilder;
mod paths;
mod pdf;
mod rendering;
mod svgpath;
mod text;

use crate::model::{FontData, Resources, Slide, SlideDeck, SlideId, Step};
pub(crate) use pdf::PdfGlobalInfo;

use crate::render::counters::{compute_counters, CountersMap};
use crate::render::pagebuilder::PageBuilder;
use crate::render::rendering::render_to_canvas;
use crate::render::text::{GlobalTextCache, TextCache, TextContext};
use itertools::Itertools;
use parley::{FontContext, LayoutContext};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;
use std::sync::Arc;

pub(crate) struct ThreadLocalResources {
    pub text_context: TextContext,
}

pub(crate) struct RenderConfig<'a> {
    pub resources: &'a Resources,
    pub thread_resources: &'a mut ThreadLocalResources,
    pub slide: &'a Slide,
    pub slide_id: SlideId,
    pub step: &'a Step,
    pub text_cache: TextCache<'a>,
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
    pub compression_level: u8,
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

#[allow(clippy::too_many_arguments)]
fn render_slide_step(
    resources: &Resources,
    thread_resources: &mut ThreadLocalResources,
    global_text_cache: &GlobalTextCache,
    builder: &PageBuilder,
    slide_id: SlideId,
    slide: &Slide,
    step: &Step,
    default_font: &Arc<FontData>,
    counter_values: &CountersMap,
) -> crate::Result<()> {
    log::debug!("Rendering slide {}/{}", slide_id, step);
    let mut render_cfg = RenderConfig {
        resources,
        thread_resources,
        slide,
        slide_id,
        step,
        default_font,
        counter_values,
        text_cache: TextCache::new(global_text_cache),
    };
    let canvas = render_to_canvas(&mut render_cfg);
    let counter = render_cfg.counter_values.get("global").unwrap();
    let page_idx = counter
        .indices
        .get(&(render_cfg.slide_id, render_cfg.step.clone()))
        .unwrap()
        .page_idx;
    builder.add_page(slide_id, step, page_idx, canvas, render_cfg.resources)
}

pub(crate) fn render_slide_deck(
    slide_deck: &SlideDeck,
    resources: &Resources,
    output_cfg: &OutputConfig,
    verbose_level: VerboseLevel,
    n_threads: Option<usize>,
) -> crate::Result<Vec<(usize, Step, Vec<u8>)>> {
    let start_time = std::time::Instant::now();
    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(n_threads) = n_threads {
        thread_pool_builder = thread_pool_builder.num_threads(n_threads);
    }
    let thread_pool = thread_pool_builder.build().unwrap();
    thread_pool.install(|| {
        if verbose_level.is_full() {
            println!(
                "Slides construction: {:.2}s",
                (start_time - slide_deck.creation_time).as_secs_f32()
            );
        }

        let counter_values = compute_counters(slide_deck);
        let global_counter = counter_values.get("global").unwrap();

        let builder = PageBuilder::new(
            slide_deck,
            output_cfg,
            verbose_level.is_normal_or_more(),
            global_counter.n_pages,
        )?;

        let text_cache = GlobalTextCache::default();

        let (r1, r2) = rayon::join(
            || {
                let tasks = slide_deck
                    .slides
                    .iter()
                    .enumerate()
                    .flat_map(|(slide_idx, slide)| {
                        slide
                            .visible_steps()
                            .map(move |step| (slide_idx, slide, step))
                    })
                    .collect_vec();
                tasks.into_par_iter().try_for_each_init(
                    || ThreadLocalResources {
                        text_context: TextContext {
                            layout_cx: LayoutContext::new(),
                            font_cx: FontContext {
                                collection: resources.font_context.collection.clone(),
                                source_cache: resources.font_context.source_cache.clone(),
                            },
                        },
                    },
                    |thread_resources, (slide_idx, slide, step)| {
                        render_slide_step(
                            resources,
                            thread_resources,
                            &text_cache,
                            &builder,
                            slide_idx as SlideId,
                            slide,
                            step,
                            &slide_deck.default_font,
                            &counter_values,
                        )
                    },
                )
            },
            || builder.other_tasks(),
        );
        r1?;
        r2?;

        let result_data = builder.finish()?;

        if verbose_level.is_full() {
            let render_end_time = std::time::Instant::now();
            println!(
                "Total rendering time: {:.2}s",
                (render_end_time - start_time).as_secs_f32()
            );
        }
        Ok(result_data)
    })
}
