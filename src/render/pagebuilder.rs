use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;
use crate::common::Step;
use crate::model::{Resources, SlideDeck, SlideId};
use crate::render::canvas::Canvas;
use crate::render::{OutputConfig, OutputFormat, PdfBuilder};
use indicatif::ProgressBar;
use pdf_writer::Finish;
use resvg::{tiny_skia, usvg};
use std::cell::RefCell;

pub(crate) enum PageWriter {
    Pdf(RefCell<PdfBuilder>),
    Svg,
    Png,
}

pub(crate) struct PageBuilder<'a> {
    writer: PageWriter,
    output_path: Option<&'a std::path::Path>,
    progress_bar: Option<ProgressBar>,
    n_pages: u32,
    result_data: RefCell<Vec<(usize, usize, Vec<u8>)>>,
}

impl<'a> PageBuilder<'a> {
    pub fn new(
        _slide_deck: &SlideDeck,
        output_config: &'a OutputConfig,
        progress_bar: Option<ProgressBar>,
        n_pages: u32,
    ) -> crate::Result<Self> {
        let mut result_data = Vec::new();
        Ok(PageBuilder {
            writer: match output_config.format {
                OutputFormat::Pdf => PageWriter::Pdf(RefCell::new(PdfBuilder::new(n_pages))),
                OutputFormat::Svg => {
                    if output_config.path.is_none() {
                        result_data.resize(n_pages as usize, (0, 0, Vec::new()))
                    }
                    if let Some(path) = output_config.path {
                        log::debug!("Ensuring output directory for SVG: {}", path.display());
                        ensure_directory(path).map_err(|e| {
                            NelsieError::Generic(format!(
                                "Cannot create directory for SVG output files: {}: {}",
                                path.display(),
                                e
                            ))
                        })?;
                    }
                    PageWriter::Svg
                }
                OutputFormat::Png => {
                    if output_config.path.is_none() {
                        result_data.resize(n_pages as usize, (0, 0, Vec::new()))
                    }
                    if let Some(path) = output_config.path {
                        log::debug!("Ensuring output directory for PNG: {}", path.display());
                        ensure_directory(path).map_err(|e| {
                            NelsieError::Generic(format!(
                                "Cannot create directory for PNG output files: {}: {}",
                                path.display(),
                                e
                            ))
                        })?;
                    }
                    PageWriter::Png
                }
            },
            output_path: output_config.path,
            progress_bar,
            n_pages,
            result_data: RefCell::new(result_data),
        })
    }

    pub fn finish(self) -> crate::Result<Vec<(usize, usize, Vec<u8>)>> {
        match (self.writer, self.output_path) {
            (PageWriter::Pdf(builder), Some(path)) => builder.into_inner().write(path)?,
            _ => { /* Do nothing */ }
        }
        if let Some(bar) = self.progress_bar {
            bar.finish();
        }
        Ok(self.result_data.into_inner())
    }

    pub fn add_page(
        &self,
        slide_id: SlideId,
        step: Step,
        page_id: u32,
        canvas: Canvas,
        resources: &Resources,
    ) -> crate::Result<()> {
        let data = match &self.writer {
            PageWriter::Pdf(writer) => {
                write_pdf_page(&mut writer.borrow_mut(), page_id, canvas, resources)?
            }
            PageWriter::Svg => write_svg_page(
                self.output_path,
                slide_id,
                step,
                page_id,
                canvas,
                self.n_pages,
            )?,
            PageWriter::Png => write_png_page(
                self.output_path,
                slide_id,
                step,
                page_id,
                canvas,
                resources,
                self.n_pages,
            )?,
        };
        if let Some(data) = data {
            self.result_data.borrow_mut()[page_id as usize] =
                (slide_id as usize, step as usize, data);
        }
        if let Some(bar) = &self.progress_bar {
            bar.inc(1);
        }
        Ok(())
    }
}

fn path_name(
    slide_id: SlideId,
    step: Step,
    page_idx: u32,
    extension: &str,
    n_pages: u32,
) -> String {
    let padding = n_pages.to_string().len();
    format!(
        "{:0padding$}-{}-{}.{}",
        page_idx,
        slide_id,
        step,
        extension,
        padding = padding,
    )
}

fn write_svg_page(
    path: Option<&std::path::Path>,
    slide_id: SlideId,
    step: Step,
    page_idx: u32,
    canvas: Canvas,
    n_pages: u32,
) -> crate::Result<Option<Vec<u8>>> {
    let data = canvas.into_svg()?;

    if let Some(path) = path {
        let final_path = path.join(path_name(slide_id, step, page_idx, "svg", n_pages));
        std::fs::write(final_path, data)?;
        Ok(None)
    } else {
        Ok(Some(data.into_bytes()))
    }
}

fn write_pdf_page(
    pdf_builder: &mut PdfBuilder,
    page_idx: u32,
    canvas: Canvas,
    resources: &Resources,
) -> crate::Result<Option<Vec<u8>>> {
    let data = canvas.into_svg()?;
    let tree = usvg::Tree::from_str(&data, &usvg::Options::default(), &resources.font_db)?;
    pdf_builder.add_page_from_svg(page_idx as usize, tree, &resources.font_db);
    Ok(None)
}

fn write_png_page(
    path: Option<&std::path::Path>,
    slide_id: SlideId,
    step: Step,
    page_idx: u32,
    canvas: Canvas,
    resources: &Resources,
    n_pages: u32,
) -> crate::Result<Option<Vec<u8>>> {
    let data = canvas.into_svg()?;
    let tree = usvg::Tree::from_str(&data, &usvg::Options::default(), &resources.font_db)?;
    let zoom = 1.0;
    let pixmap_size = tree.size().to_int_size().scale_by(zoom).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);
    resvg::render(&tree, render_ts, &mut pixmap.as_mut());

    let output = pixmap
        .encode_png()
        .map_err(|e| NelsieError::Generic(e.to_string()))?;

    if let Some(path) = path {
        let final_path = path.join(path_name(slide_id, step, page_idx, "png", n_pages));
        std::fs::write(final_path, output)?;
        Ok(None)
    } else {
        Ok(Some(output))
    }
}
