mod core;
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

fn render_slide(
    resources: &Resources,
    output_cfg: &OutputConfig,
    slide_idx: usize,
    slide: &Slide,
    default_font: &Arc<FontData>,
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
            };
            render_slide_step(&render_cfg)
        })
        .collect()
}

pub(crate) fn render_slide_deck(
    slide_deck: &SlideDeck,
    resources: &Resources,
    output_cfg: &OutputConfig,
) -> crate::Result<Vec<(usize, usize, Vec<u8>)>> {
    let start_time = std::time::Instant::now();
    println!(
        "Slides construction: {:.2}s",
        (start_time - slide_deck.creation_time).as_secs_f32()
    );

    let mut pdf_builder = if let OutputFormat::Pdf = output_cfg.format {
        let n_steps = slide_deck.slides.iter().map(|s| s.n_steps).sum();
        Some(PdfBuilder::new(n_steps))
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

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        for (step_idx, result) in render_slide(
            resources,
            output_cfg,
            slide_idx,
            slide,
            &slide_deck.default_font,
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
        }
    }
    println!("Pdf time: {:.2}s", pdf_compose_time.as_secs_f32());

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

    let render_end_time = std::time::Instant::now();
    println!(
        "Total rendering time: {:.2}s",
        (render_end_time - start_time).as_secs_f32()
    );

    Ok(result_data)
}
