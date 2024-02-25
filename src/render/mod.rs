mod core;
mod counters;
mod image;
mod layout;
mod paths;
mod pdf;
mod rendering;
mod text;

use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;

use crate::model::{FontData, Resources, Slide, SlideDeck};
use crate::render::core::RenderingResult;
pub(crate) use core::{render_slide_step, RenderConfig};
pub(crate) use pdf::PdfBuilder;

use crate::render::counters::{compute_counters, CountersMap};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub(crate) enum OutputFormat {
    Pdf,
    Svg,
    Png,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Svg => "svg",
            OutputFormat::Png => "png",
        }
    }
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
    output_cfg: &OutputConfig,
    slide_idx: usize,
    slide: &Slide,
    default_font: &Arc<FontData>,
    counter_values: &CountersMap,
) -> crate::Result<Vec<RenderingResult>> {
    log::debug!("Rendering slide {}", slide_idx);
    (1..=slide.n_steps)
        .map(|step| {
            let render_cfg = RenderConfig {
                resources,
                slide,
                slide_idx,
                step,
                default_font,
                output_format: output_cfg.format,
                output_path: output_cfg.path,
                counter_values,
            };
            render_slide_step(&render_cfg)
        })
        .collect()
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
    let n_pages = counter_values.get("global").unwrap().n_pages;
    let mut pdf_builder = if let OutputFormat::Pdf = output_cfg.format {
        Some(PdfBuilder::new(n_pages))
    } else {
        if let Some(dir) = output_cfg.path {
            log::debug!("Ensuring output directory: {}", dir.display());
            ensure_directory(dir).map_err(|e| {
                NelsieError::Generic(format!(
                    "Cannot create directory for output files: {}: {}",
                    dir.display(),
                    e
                ))
            })?;
        }
        None
    };

    let mut result_data = Vec::new();

    let mut pdf_compose_time = Duration::ZERO;

    let progress_bar = if verbose_level.is_normal_or_more() {
        Some(indicatif::ProgressBar::new(n_pages.into()))
    } else {
        None
    };

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        for (step_idx, result) in render_slide(
            resources,
            output_cfg,
            slide_idx,
            slide,
            &slide_deck.default_font,
            &counter_values,
        )?
        .into_iter()
        .enumerate()
        {
            match result {
                RenderingResult::None => { /* Do nothing */ }
                RenderingResult::Tree(tree) => {
                    let s = std::time::Instant::now();
                    pdf_builder.as_mut().unwrap().add_page_from_svg(tree);
                    pdf_compose_time += std::time::Instant::now() - s;
                }
                RenderingResult::BytesData(data) => {
                    result_data.push((slide_idx, step_idx, data));
                }
            }
            if let Some(bar) = &progress_bar {
                bar.inc(1);
            }
        }
    }
    if let Some(bar) = &progress_bar {
        bar.finish();
    }
    if let Some(builder) = pdf_builder {
        if let Some(path) = output_cfg.path {
            builder.write(path).map_err(|e| {
                NelsieError::Generic(format!("Writing PDF file {}: {}", path.display(), e))
            })?;
        } else {
            // TODO: Introduce a public method for getting data without writing in svg2pdf
            //result_data.push(builder.finish());
        }
    }

    if verbose_level.is_full() {
        let render_end_time = std::time::Instant::now();
        println!(
            "Total rendering time: {:.2}s",
            (render_end_time - start_time).as_secs_f32()
        );
        println!("   |--- Pdf time: {:.2}s", pdf_compose_time.as_secs_f32());
    }

    Ok(result_data)
}
