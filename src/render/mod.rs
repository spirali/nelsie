mod core;
mod globals;
mod image;
mod layout;
mod paths;
mod pdf;
mod rendering;
mod text;

use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;
use crate::model::{Resources, Slide, SlideDeck};
pub(crate) use core::{render_slide_step, RenderConfig};
pub(crate) use pdf::PdfBuilder;
use std::path::Path;
pub(crate) use text::check_fonts;
use usvg::fontdb;

pub(crate) struct OutputConfig<'a> {
    pub output_pdf: Option<&'a Path>,
    pub output_svg: Option<&'a Path>,
    pub output_png: Option<&'a Path>,
}

fn render_slide(
    resources: &Resources,
    output_cfg: &OutputConfig,
    slide_idx: usize,
    slide: &Slide,
) -> crate::Result<Vec<usvg::Tree>> {
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
                resources,
                output_svg: output_svg.as_deref(),
                output_png: output_png.as_deref(),
                slide,
                step,
            };
            render_slide_step(&render_cfg)
        })
        .collect()
}

pub(crate) fn render_slide_deck(
    slide_deck: &SlideDeck,
    resources: &Resources,
    output_cfg: &OutputConfig,
) -> crate::Result<()> {
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

    // let mut font_db = fontdb::Database::new();
    // font_db.load_system_fonts();

    //let loaded_images = load_image_in_deck(&font_db, &mut slide_deck)?;
    //check_fonts(&font_db, &slide_deck)?;

    let n_steps = slide_deck.slides.iter().map(|s| s.n_steps).sum();
    let mut pdf_builder = output_cfg.output_pdf.map(|_| PdfBuilder::new(n_steps));

    for (slide_idx, slide) in slide_deck.slides.iter().enumerate() {
        for tree in render_slide(&resources, output_cfg, slide_idx, slide)? {
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
