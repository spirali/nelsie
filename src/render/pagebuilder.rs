use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;
use crate::model::{LoadedImage, LoadedImageData, Resources, SlideDeck, SlideId, Step};
use crate::render::canvas::Canvas;
use crate::render::canvas_pdf::PdfImageCache;
use crate::render::pdf::{image_to_pdf_chunk, PdfPage};
use crate::render::{OutputConfig, OutputFormat, PdfBuilder};
use by_address::ByAddress;
use indicatif::ProgressBar;
use itertools::Itertools;

use pdf_writer::{Chunk, Finish};
use resvg::{tiny_skia, usvg};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub(crate) struct PdfWriterData {
    pages: Mutex<Vec<Option<PdfPage>>>,
    cache: PdfImageCache,
    pdf_builder: PdfBuilder,
    images: Vec<Arc<LoadedImage>>,
    image_chunks: Mutex<Vec<Chunk>>,
}

pub(crate) enum PageWriter {
    Pdf(PdfWriterData),
    Svg(Mutex<Vec<(usize, Step, Vec<u8>)>>),
    Png(Mutex<Vec<(usize, Step, Vec<u8>)>>),
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
                    let (cache, images) = collect_image_cache(slide_deck, &mut pdf_builder);
                    let mut pages = Vec::with_capacity(n_pages as usize);
                    for _ in 0..n_pages {
                        pages.push(None);
                    }
                    PageWriter::Pdf(PdfWriterData {
                        pages: Mutex::new(pages),
                        cache,
                        pdf_builder,
                        images,
                        image_chunks: Mutex::new(Vec::new()),
                    })
                }
                OutputFormat::Svg => {
                    let mut result_data = Vec::new();
                    if output_config.path.is_none() {
                        result_data.resize(n_pages as usize, (0, Step::default(), Vec::new()))
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
                        result_data.resize(n_pages as usize, (0, Step::default(), Vec::new()))
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

    pub fn other_tasks(&self) -> crate::Result<()> {
        match &self.writer {
            PageWriter::Pdf(pdf_writer) => {
                let chunks = precompute_image_cache(&pdf_writer.cache, &pdf_writer.images);
                *pdf_writer.image_chunks.lock().unwrap() = chunks;
            }
            PageWriter::Svg(_) | PageWriter::Png(_) => {}
        };
        Ok(())
    }

    pub fn finish(self) -> crate::Result<Vec<(usize, Step, Vec<u8>)>> {
        let result = match (self.writer, self.output_path) {
            (PageWriter::Pdf(mut data), Some(path)) => {
                let image_chunks = data.image_chunks.into_inner().unwrap();
                let pages = data.pages.into_inner().unwrap();

                for chunk in image_chunks {
                    data.pdf_builder.add_chunk_direct(chunk);
                }

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
        step: &Step,
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
                let data = write_svg_page(self.output_path, page_id, canvas, self.n_pages)?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_id as usize] = (slide_id as usize, step.clone(), data);
                }
            }
            PageWriter::Png(output) => {
                let data =
                    write_png_page(self.output_path, page_id, canvas, resources, self.n_pages)?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_id as usize] = (slide_id as usize, step.clone(), data);
                }
            }
        };
        if let Some(bar) = &self.progress_bar {
            bar.inc(1);
        }
        Ok(())
    }
}

fn path_name(page_idx: u32, extension: &str, n_pages: u32) -> String {
    let padding = n_pages.to_string().len();
    format!("{:0padding$}.{}", page_idx, extension, padding = padding,)
}

fn write_svg_page(
    path: Option<&std::path::Path>,
    page_idx: u32,
    canvas: Canvas,
    n_pages: u32,
) -> crate::Result<Option<Vec<u8>>> {
    let data = canvas.into_svg()?;

    if let Some(path) = path {
        let final_path = path.join(path_name(page_idx, "svg", n_pages));
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
        let final_path = path.join(path_name(page_idx, "png", n_pages));
        std::fs::write(final_path, output)?;
        Ok(None)
    } else {
        Ok(Some(output))
    }
}

pub fn collect_image_cache(
    deck: &SlideDeck,
    pdf_builder: &mut PdfBuilder,
) -> (PdfImageCache, Vec<Arc<LoadedImage>>) {
    let mut image_set = HashSet::new();
    for slide in &deck.slides {
        slide.node.collect_images(&mut image_set);
    }
    let mut image_vec: Vec<Arc<LoadedImage>> = image_set
        .into_iter()
        .filter_map(|img| match img.data {
            LoadedImageData::Png(_) | LoadedImageData::Jpeg(_) | LoadedImageData::Ora(_) => {
                Some(img.0)
            }
            LoadedImageData::Svg(_) => None,
        })
        .collect_vec();
    image_vec.sort_unstable_by_key(|i| i.image_id);
    let mut cache = PdfImageCache::new();
    for image in &image_vec {
        match &image.data {
            LoadedImageData::Png(data) => {
                cache.insert(ByAddress(data.clone()), pdf_builder.ref_bump());
                pdf_builder.ref_bump(); // Reserve ID for transparency layer
            }
            LoadedImageData::Jpeg(data) => {
                cache.insert(ByAddress(data.clone()), pdf_builder.ref_bump());
            }
            LoadedImageData::Svg(_) => {
                unreachable!()
            }
            LoadedImageData::Ora(ora) => {
                for layer in &ora.layers {
                    cache.insert(ByAddress(layer.data.clone()), pdf_builder.ref_bump());
                    pdf_builder.ref_bump(); // Reserve ID for transparency layer
                }
            }
        };
    }
    (cache, image_vec)
}

pub fn precompute_image_cache(cache: &PdfImageCache, images: &[Arc<LoadedImage>]) -> Vec<Chunk> {
    images
        .into_par_iter()
        .map(|image| match &image.data {
            LoadedImageData::Png(data) => {
                let id = *cache.get(&ByAddress(data.clone())).unwrap();
                vec![image_to_pdf_chunk(
                    image::ImageFormat::Png,
                    data,
                    id,
                    Some(pdf_writer::Ref::new(id.get() + 1)),
                )]
            }
            LoadedImageData::Jpeg(data) => {
                let id = cache.get(&ByAddress(data.clone())).unwrap();
                vec![image_to_pdf_chunk(
                    image::ImageFormat::Jpeg,
                    data,
                    *id,
                    None,
                )]
            }
            LoadedImageData::Svg(_) => {
                unreachable!()
            }
            LoadedImageData::Ora(ora) => ora
                .layers
                .par_iter()
                .map(|layer| {
                    let id = cache.get(&ByAddress(layer.data.clone())).unwrap();
                    image_to_pdf_chunk(
                        image::ImageFormat::Png,
                        &layer.data,
                        *id,
                        Some(pdf_writer::Ref::new(id.get() + 1)),
                    )
                })
                .collect(),
        })
        .flatten()
        .collect()
}
