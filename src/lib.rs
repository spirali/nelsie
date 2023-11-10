mod common;
mod model;
mod render;

use crate::common::fileutils::ensure_directory;
use crate::model::{Node, Slide, SlideDeck};
use crate::render::{render_slide_step, GlobalResources, PdfBuilder, RenderConfig};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NelsieError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Error: {0}")]
    GenericError(String),
}

pub struct OutputConfig<'a> {
    pub output_pdf: Option<&'a Path>,
    pub output_svg: Option<&'a Path>,
    pub output_png: Option<&'a Path>,
}

pub type Result<T> = std::result::Result<T, NelsieError>;

impl From<serde_json::error::Error> for NelsieError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::DeserializationError(e.to_string())
    }
}

fn parse_slide_deck(data: &str) -> Result<SlideDeck> {
    serde_json::from_str(data).map_err(|e| e.into())
}

fn render_slide(
    global_res: &GlobalResources,
    output_cfg: &OutputConfig,
    slide_idx: usize,
    slide: &Slide,
) -> Result<Vec<usvg::Tree>> {
    log::debug!("Rendering slide {}", slide_idx);
    (1..=slide.n_steps)
        .map(|step| {
            let output_svg = output_cfg
                .output_svg
                .map(|p| p.join(format!("{}-{}.svg", slide_idx, step)));
            let output_png = output_cfg
                .output_png
                .map(|p| p.join(format!("{}-{}.png", slide_idx, step)));

            let render_cfg = RenderConfig {
                global_res,
                output_svg: output_svg.as_deref(),
                output_png: output_png.as_deref(),
                slide,
                step,
            };
            render_slide_step(&render_cfg)
        })
        .collect()
}

pub fn render_slide_deck(data: &str, output_cfg: &OutputConfig) -> Result<()> {
    log::debug!("Input received:\n{}", data);
    let slide_deck = parse_slide_deck(data)?;

    if let Some(dir) = output_cfg.output_svg {
        log::debug!("Ensuring SVG output directory: {}", dir.display());
        ensure_directory(dir).map_err(|e| {
            NelsieError::GenericError(format!(
                "Cannot create directory for SVG files: {}: {}",
                dir.display(),
                e
            ))
        })?;
    }

    if let Some(dir) = output_cfg.output_png {
        log::debug!("Ensuring PNG output directory: {}", dir.display());
        ensure_directory(dir).map_err(|e| {
            NelsieError::GenericError(format!(
                "Cannot create directory for PNG files: {}: {}",
                dir.display(),
                e
            ))
        })?;
    }

    let global_res = GlobalResources::new();

    let n_steps = slide_deck.slides.iter().map(|s| s.n_steps).sum();
    let mut pdf_builder = output_cfg.output_pdf.map(|_| PdfBuilder::new(n_steps));

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        for tree in render_slide(&global_res, output_cfg, slide_idx, slide)? {
            if let Some(builder) = &mut pdf_builder {
                builder.add_page_from_svg(tree);
            }
        }
    }
    if let Some(builder) = pdf_builder {
        let path = output_cfg.output_pdf.unwrap();
        builder.write(path).map_err(|e| {
            NelsieError::GenericError(format!("Writing PDF file {}: {}", path.display(), e))
        })?;
    }

    Ok(())
}