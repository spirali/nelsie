mod model;
mod common;
mod render;

use std::path::{Path, PathBuf};
use crate::model::{Node, Slide, SlideDeck};
use crate::render::{GlobalResources, render_slide_step, RenderConfig};
use thiserror::Error;
use crate::common::fileutils::ensure_directory;


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

fn render_slide(global_res: &GlobalResources, output_cfg: &OutputConfig, slide_idx: usize, slide: &Slide) -> Result<()> {
    log::debug!("Rendering slide {}", slide_idx);
    let step = 1;

    let output_svg = output_cfg.output_svg.map(|p| p.join(format!("{}-{}.svg", slide_idx, step)));
    let output_png = output_cfg.output_png.map(|p| p.join(format!("{}-{}.png", slide_idx, step)));

    let render_cfg = RenderConfig { global_res, output_svg: output_svg.as_deref(), output_png: output_png.as_deref(), slide, step };
    render_slide_step(&render_cfg)?;
    Ok(())
}

pub fn render_slide_deck(data: &str, output_cfg: &OutputConfig) -> Result<()> {
    log::debug!("Input received:\n{}", data);
    let slide_deck = parse_slide_deck(data)?;

    if let Some(dir) = output_cfg.output_svg {
        log::debug!("Ensuring SVG output directory: {}", dir.display());
        ensure_directory(dir).map_err(|e|
            NelsieError::GenericError(format!("Cannot create directory for SVG files: {}: {}", dir.display(), e))
        )?;
    }

    if let Some(dir) = output_cfg.output_png {
        log::debug!("Ensuring PNG output directory: {}", dir.display());
        ensure_directory(dir).map_err(|e|
            NelsieError::GenericError(format!("Cannot create directory for PNG files: {}: {}", dir.display(), e))
        )?;
    }

    let global_res = GlobalResources::new();
    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        render_slide(&global_res, output_cfg, slide_idx, slide)?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
