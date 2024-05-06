use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;
use crate::common::Step;
use crate::model::{LoadedImageData, Resources, SlideDeck, SlideId};
use crate::render::canvas::Canvas;
use crate::render::canvas_pdf::PdfImageCache;
use crate::render::pdf::{image_to_pdf_chunk, PdfPage};
use crate::render::{OutputConfig, OutputFormat, PdfBuilder};
use by_address::ByAddress;
use indicatif::ProgressBar;
use itertools::Itertools;

use pdf_writer::Finish;
use resvg::{tiny_skia, usvg};

use std::collections::HashSet;
use std::sync::Mutex;

pub(crate) struct PdfWriterData {
    pages: Mutex<Vec<Option<PdfPage>>>,
    cache: PdfImageCache,
    pdf_builder: PdfBuilder,
}

pub(crate) enum PageWriter {
    Pdf(PdfWriterData),
    Svg(Mutex<Vec<(usize, usize, Vec<u8>)>>),
    Png(Mutex<Vec<(usize, usize, Vec<u8>)>>),
}

pub(crate) struct PageBuilder<'a> {
    writer: PageWriter,
    output_path: Option<&'a std::path::Path>,
    progress_bar: Option<ProgressBar>,
    n_pages: u32,
}

impl<'a> PageBuilder<'a> {
    pub fn new(
        slide_deck: &SlideDeck,
        output_config: &'a OutputConfig,
        progress_bar: Option<ProgressBar>,
        n_pages: u32,
    ) -> crate::Result<Self> {
        Ok(PageBuilder {
            writer: match output_config.format {
                OutputFormat::Pdf => {
                    let mut pdf_builder = PdfBuilder::new(n_pages);
                    let cache = collect_image_cache(slide_deck, &mut pdf_builder);
                    let mut pages = Vec::with_capacity(n_pages as usize);
                    for _ in 0..n_pages {
                        pages.push(None);
                    }
                    PageWriter::Pdf(PdfWriterData {
                        pages: Mutex::new(pages),
                        cache,
                        pdf_builder,
                    })
                }
                OutputFormat::Svg => {
                    let mut result_data = Vec::new();
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
                    PageWriter::Svg(Mutex::new(result_data))
                }
                OutputFormat::Png => {
                    let mut result_data = Vec::new();
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
                    PageWriter::Png(Mutex::new(result_data))
                }
            },
            output_path: output_config.path,
            progress_bar,
            n_pages,
        })
    }

    pub fn finish(self) -> crate::Result<Vec<(usize, usize, Vec<u8>)>> {
        let result = match (self.writer, self.output_path) {
            (PageWriter::Pdf(mut data), Some(path)) => {
                let pages = data.pages.into_inner().unwrap();
                for (page_idx, page) in pages.into_iter().enumerate() {
                    let page = page.unwrap();
                    data.pdf_builder.add_page(page_idx, page);
                }
                data.pdf_builder.write(path)?;
                Vec::new()
            }
            (PageWriter::Png(result), None) | (PageWriter::Svg(result), None) => {
                result.into_inner().unwrap()
            }
            _ => Vec::new(),
        };
        if let Some(bar) = self.progress_bar {
            bar.finish();
        }
        Ok(result)
    }

    pub fn add_page(
        &self,
        slide_id: SlideId,
        step: Step,
        page_id: u32,
        canvas: Canvas,
        resources: &Resources,
    ) -> crate::Result<()> {
        match &self.writer {
            PageWriter::Pdf(data) => {
                let page = canvas.into_pdf_page(resources, &data.cache)?;
                data.pages.lock().unwrap()[page_id as usize] = Some(page);
            }
            PageWriter::Svg(output) => {
                let data = write_svg_page(
                    self.output_path,
                    slide_id,
                    step,
                    page_id,
                    canvas,
                    self.n_pages,
                )?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_id as usize] = (slide_id as usize, step as usize, data);
                }
            }
            PageWriter::Png(output) => {
                let data = write_png_page(
                    self.output_path,
                    slide_id,
                    step,
                    page_id,
                    canvas,
                    resources,
                    self.n_pages,
                )?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_id as usize] = (slide_id as usize, step as usize, data);
                }
            }
        };
        if let Some(bar) = &self.progress_bar {
            bar.inc(1);
        }
        Ok(())
    }
}

fn path_name(
    _slide_id: SlideId,
    _step: Step,
    page_idx: u32,
    extension: &str,
    n_pages: u32,
) -> String {
    let padding = n_pages.to_string().len();
    format!("{:0padding$}.{}", page_idx, extension, padding = padding,)
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

// fn write_pdf_page(
//     page_idx: u32,
//     canvas: Canvas,
//     resources: &Resources,
//     cache: &PdfImageCache,
// ) -> crate::Result<Option<Vec<u8>>> {
//     let width = canvas.width();
//     let height = canvas.height();
//     let bg_color = canvas.bg_color;
//
//     canvas.into_pdf_page(resources, cache)?;
//
//     // let data = canvas.into_svg()?;
//     //
//     // let tree = usvg::Tree::from_str(&data, &usvg::Options::default(), &resources.font_db)?;
//     // let (svg_chunk, svg_id) = svg2pdf::to_chunk(
//     //     &tree,
//     //     svg2pdf::ConversionOptions::default(),
//     //     &resources.font_db,
//     // );
//     // let svg_id = pdf_builder.add_chunk(svg_chunk, svg_id);
//     // let refs = vec![(Rectangle::new(0.0, 0.0, width, height), svg_id)];
//     pdf_builder.add_page_from_svg(page_idx as usize, width, height, bg_color, &refs);
//     Ok(None)
// }

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

pub fn collect_image_cache(deck: &SlideDeck, pdf_builder: &mut PdfBuilder) -> PdfImageCache {
    let mut image_set = HashSet::new();
    for slide in &deck.slides {
        slide.node.collect_images(&mut image_set);
    }
    let mut image_vec = image_set.into_iter().collect_vec();
    image_vec.sort_unstable_by_key(|i| i.image_id);
    let mut chunks = Vec::new();
    for image in image_vec {
        match &image.data {
            LoadedImageData::Png(data) => {
                let (chunk, id) = image_to_pdf_chunk(
                    image::ImageFormat::Png,
                    data,
                    pdf_builder.ref_bump(),
                    pdf_builder.ref_bump(),
                );
                chunks.push((ByAddress::from(data.clone()), chunk, id))
            }
            LoadedImageData::Gif(_) => {}
            LoadedImageData::Jpeg(data) => {
                let (chunk, id) = image_to_pdf_chunk(
                    image::ImageFormat::Jpeg,
                    data,
                    pdf_builder.ref_bump(),
                    pdf_builder.ref_bump(),
                );
                chunks.push((ByAddress::from(data.clone()), chunk, id))
            }
            LoadedImageData::Svg(_) => {}
            LoadedImageData::Ora(ora) => {
                for layer in &ora.layers {
                    let (chunk, id) = image_to_pdf_chunk(
                        image::ImageFormat::Png,
                        &layer.data,
                        pdf_builder.ref_bump(),
                        pdf_builder.ref_bump(),
                    );
                    chunks.push((ByAddress::from(layer.data.clone()), chunk, id))
                }
            }
        }
    }

    let mut cache = PdfImageCache::new();
    for (key, chunk, id) in chunks {
        pdf_builder.add_chunk_direct(chunk);
        cache.insert(key, id);
    }
    cache
}
