mod common;
mod model;
mod render;

use crate::common::fileutils::ensure_directory;
use crate::model::{Slide, SlideDeck};
use crate::render::{
    load_image_in_deck, render_slide_step, GlobalResources, PdfBuilder, RenderConfig,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use thiserror::Error;
use usvg::fontdb;

#[derive(Debug, Error)]
pub enum NelsieError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    XmlError(#[from] roxmltree::Error),
    #[error(transparent)]
    SvgError(#[from] usvg::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
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

// impl From<roxmltree::Error> for NelsieError {
//     fn from(e: roxmltree::Error) -> Self {
//         Self::XmlError(e)
//     }
// }

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
    let mut slide_deck = parse_slide_deck(data)?;

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

    let mut font_db = fontdb::Database::new();
    font_db.load_system_fonts();

    let loaded_images = load_image_in_deck(&font_db, &mut slide_deck)?;

    let global_res = GlobalResources::new(font_db, loaded_images);

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
