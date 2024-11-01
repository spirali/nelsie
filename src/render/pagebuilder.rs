use crate::common::error::NelsieError;
use crate::common::fileutils::ensure_directory;
use crate::model::{LoadedImage, LoadedImageData, Resources, SlideDeck, SlideId, Step};
use crate::render::canvas::Canvas;
use crate::render::canvas_pdf::PdfImageCache;
use crate::render::pdf::image_to_pdf_chunk;
use crate::render::{OutputConfig, OutputFormat, PdfGlobalInfo};
use by_address::ByAddress;
use indicatif::ProgressBar;
use itertools::Itertools;

use pdf_writer::{Chunk, Finish};
use resvg::{tiny_skia, usvg};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub(crate) struct PdfWriterData {
    chunks: Mutex<Vec<Chunk>>,
    cache: PdfImageCache,
    images: Vec<Arc<LoadedImage>>,
    pdf_ginfo: PdfGlobalInfo,
    pdf: Mutex<pdf_writer::Pdf>,
}

impl PdfWriterData {
    pub fn add_chunk(&self, chunk: Chunk) {
        if let Ok(mut pdf) = self.pdf.try_lock() {
            pdf.extend(&chunk);
            let chunks = {
                let mut chunks = self.chunks.lock().unwrap();
                std::mem::take(chunks.deref_mut())
            };
            for chunk in chunks {
                pdf.extend(&chunk);
            }
        } else {
            self.chunks.lock().unwrap().push(chunk);
        }
    }
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
    compression_level: u8,
}

impl<'a> PageBuilder<'a> {
    pub fn new(
        slide_deck: &SlideDeck,
        output_config: &'a OutputConfig,
        progress_bar: Option<ProgressBar>,
        n_pages: u32,
    ) -> crate::Result<Self> {
        Ok(PageBuilder {
            compression_level: output_config.compression_level,
            writer: match output_config.format {
                OutputFormat::Pdf => {
                    let mut pdf = pdf_writer::Pdf::new();
                    let mut pdf_ginfo = PdfGlobalInfo::new(&mut pdf, n_pages);
                    let (cache, images) = collect_image_cache(slide_deck, &mut pdf_ginfo);
                    let pages = Vec::with_capacity(n_pages as usize);
                    PageWriter::Pdf(PdfWriterData {
                        chunks: Mutex::new(pages),
                        cache,
                        pdf_ginfo,
                        images,
                        pdf: Mutex::new(pdf),
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
                precompute_image_cache(pdf_writer);
            }
            PageWriter::Svg(_) | PageWriter::Png(_) => {}
        };
        Ok(())
    }

    pub fn finish(self) -> crate::Result<Vec<(usize, Step, Vec<u8>)>> {
        let result = match (self.writer, self.output_path) {
            (PageWriter::Pdf(data), Some(path)) => {
                let mut pdf = data.pdf.into_inner().unwrap();
                let chunks = data.chunks.into_inner().unwrap();
                for chunk in chunks.into_iter() {
                    pdf.extend(&chunk);
                }
                std::fs::write(path, pdf.finish())?;
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
        page_idx: u32,
        canvas: Canvas,
        resources: &Resources,
    ) -> crate::Result<()> {
        match &self.writer {
            PageWriter::Pdf(data) => {
                let page = canvas.into_pdf_page(
                    resources,
                    &mut data.pdf_ginfo.page_ref_allocator(page_idx),
                    data.pdf_ginfo.page_tree_ref(),
                    &data.cache,
                    self.compression_level,
                )?;
                data.add_chunk(page);
            }
            PageWriter::Svg(output) => {
                let data = write_svg_page(self.output_path, page_idx, canvas, self.n_pages)?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_idx as usize] = (slide_id as usize, step.clone(), data);
                }
            }
            PageWriter::Png(output) => {
                let data =
                    write_png_page(self.output_path, page_idx, canvas, resources, self.n_pages)?;
                if let Some(data) = data {
                    let mut result_data = output.lock().unwrap();
                    result_data[page_idx as usize] = (slide_id as usize, step.clone(), data);
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

fn write_png_page(
    path: Option<&std::path::Path>,
    page_idx: u32,
    canvas: Canvas,
    resources: &Resources,
    n_pages: u32,
) -> crate::Result<Option<Vec<u8>>> {
    let data = canvas.into_svg()?;
    let options = usvg::Options {
        fontdb: resources.font_db.as_ref().unwrap().clone(),
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(&data, &options)?;
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
    pdf_builder: &mut PdfGlobalInfo,
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
            LoadedImageData::Png(data) | LoadedImageData::Jpeg(data) => {
                cache.insert(
                    ByAddress(data.clone()),
                    (pdf_builder.ref_bump(), pdf_builder.ref_bump()),
                );
            }
            LoadedImageData::Svg(_) => {
                unreachable!()
            }
            LoadedImageData::Ora(ora) => {
                for layer in &ora.layers {
                    cache.insert(
                        ByAddress(layer.data.clone()),
                        (pdf_builder.ref_bump(), pdf_builder.ref_bump()),
                    );
                }
            }
        };
    }
    (cache, image_vec)
}

pub fn precompute_image_cache(pdf_writer: &PdfWriterData) {
    pdf_writer
        .images
        .par_iter()
        .for_each(|image| match &image.data {
            LoadedImageData::Png(data) => {
                let (img_ref, transparency_ref) =
                    *pdf_writer.cache.get(&ByAddress(data.clone())).unwrap();
                let chunk = image_to_pdf_chunk(
                    image::ImageFormat::Png,
                    data,
                    img_ref,
                    Some(transparency_ref),
                );
                pdf_writer.add_chunk(chunk);
            }
            LoadedImageData::Jpeg(data) => {
                let (img_ref, _mask_ref) = *pdf_writer.cache.get(&ByAddress(data.clone())).unwrap();
                let chunk = image_to_pdf_chunk(image::ImageFormat::Jpeg, data, img_ref, None);
                pdf_writer.add_chunk(chunk);
            }
            LoadedImageData::Svg(_) => {
                unreachable!()
            }
            LoadedImageData::Ora(ora) => ora.layers.par_iter().for_each(|layer| {
                let (img_id, mask_ref) = *pdf_writer
                    .cache
                    .get(&ByAddress(layer.data.clone()))
                    .unwrap();
                let chunk = image_to_pdf_chunk(
                    image::ImageFormat::Png,
                    &layer.data,
                    img_id,
                    Some(mask_ref),
                );
                pdf_writer.add_chunk(chunk);
            }),
        });
}
