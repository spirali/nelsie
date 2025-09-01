/*use itertools::Itertools;

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

#[allow(clippy::large_enum_variant)] // This is ok, because this struct is not moved anywhere
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
        has_progress_bar: bool,
        n_pages: u32,
    ) -> crate::Result<Self> {
        let (writer, progress_steps) = match output_config.format {
            OutputFormat::Pdf => {
                let mut pdf = pdf_writer::Pdf::new();
                let mut pdf_ginfo = PdfGlobalInfo::new(&mut pdf, n_pages);
                let (cache, images) = collect_image_cache(slide_deck, &mut pdf_ginfo);
                let count = images.iter().map(|img| img.image_count()).sum::<usize>() as u64
                    + n_pages as u64;
                let pages = Vec::new();
                (
                    PageWriter::Pdf(PdfWriterData {
                        chunks: Mutex::new(pages),
                        cache,
                        pdf_ginfo,
                        images,
                        pdf: Mutex::new(pdf),
                    }),
                    count,
                )
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
                (PageWriter::Svg(Mutex::new(result_data)), n_pages as u64)
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
                (PageWriter::Png(Mutex::new(result_data)), n_pages as u64)
            }
        };

        let progress_bar = if has_progress_bar {
            Some(ProgressBar::new(progress_steps))
        } else {
            None
        };

        Ok(PageBuilder {
            writer,
            compression_level: output_config.compression_level,
            progress_bar,
            n_pages,
            output_path: output_config.path,
        })
    }

    pub fn other_tasks(&self) -> crate::Result<()> {
        match &self.writer {
            PageWriter::Pdf(pdf_writer) => {
                precompute_image_cache(pdf_writer, self.progress_bar.as_ref());
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
    let image_vec: Vec<Arc<LoadedImage>> = image_set
        .into_iter()
        .filter_map(|img| match img.data {
            LoadedImageData::Png(_) | LoadedImageData::Jpeg(_) | LoadedImageData::Ora(_) => {
                Some(img.0)
            }
            LoadedImageData::Svg(_) => None,
        })
        .collect_vec();
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

pub fn precompute_image_cache(pdf_writer: &PdfWriterData, progress_bar: Option<&ProgressBar>) {
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
                if let Some(bar) = progress_bar {
                    bar.inc(1);
                }
            }
            LoadedImageData::Jpeg(data) => {
                let (img_ref, _mask_ref) = *pdf_writer.cache.get(&ByAddress(data.clone())).unwrap();
                let chunk = image_to_pdf_chunk(image::ImageFormat::Jpeg, data, img_ref, None);
                pdf_writer.add_chunk(chunk);
                if let Some(bar) = progress_bar {
                    bar.inc(1);
                }
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
                if let Some(bar) = progress_bar {
                    bar.inc(1);
                }
            }),
        });
}
*/
use crate::render::canvas::Canvas;
use crate::render::content::{Content, ContentMap};
use crate::render::layout::ComputedLayout;
use crate::{ContentId, Resources};
use resvg::{tiny_skia, usvg};
use std::sync::Mutex;

pub(crate) trait Composer: Sync + Send {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        compute_layout: &ComputedLayout,
    ) -> crate::Result<()>;
    fn preprocess_content(
        &self,
        _resources: &Resources,
        _content_id: ContentId,
        _content: &Content,
    ) -> crate::Result<()> {
        Ok(())
    }

    fn preprocessing_finished(&mut self) {}

    fn needs_image_preprocessing(&self) -> bool {
        false
    }
}

fn path_name(page_idx: usize, extension: &str, n_pages: usize) -> String {
    let padding = n_pages.ilog10() as usize + 1;
    format!("{:0padding$}.{}", page_idx, extension, padding = padding,)
}

pub(crate) struct SvgWriteComposer<'a> {
    path: &'a std::path::Path,
    n_pages: usize,
}

impl<'a> SvgWriteComposer<'a> {
    pub fn new(path: &'a std::path::Path, n_pages: usize) -> Self {
        Self { path, n_pages }
    }
}

impl Composer for SvgWriteComposer<'_> {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        _layout: &ComputedLayout,
    ) -> crate::Result<()> {
        let svg = canvas.as_svg(content_map)?;
        let final_path = self.path.join(path_name(page_idx, "svg", self.n_pages));
        std::fs::write(final_path, svg)?;
        Ok(())
    }
}

pub(crate) struct PngWriteComposer<'a> {
    resources: &'a Resources,
    path: &'a std::path::Path,
    n_pages: usize,
}

impl<'a> PngWriteComposer<'a> {
    pub fn new(resources: &'a Resources, path: &'a std::path::Path, n_pages: usize) -> Self {
        Self {
            resources,
            path,
            n_pages,
        }
    }
}

impl Composer for PngWriteComposer<'_> {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        _layout: &ComputedLayout,
    ) -> crate::Result<()> {
        let svg = canvas.as_svg(content_map)?;
        let data = svg_to_png(self.resources, &svg)?;
        let final_path = self.path.join(path_name(page_idx, "png", self.n_pages));
        std::fs::write(final_path, data)?;
        Ok(())
    }
}

pub(crate) struct SvgCollectingComposer {
    pages: Mutex<Vec<String>>,
}

impl SvgCollectingComposer {
    pub fn new(n_pages: usize) -> Self {
        Self {
            pages: Mutex::new(vec![String::new(); n_pages]),
        }
    }

    pub fn finish(self) -> Vec<String> {
        self.pages.into_inner().unwrap()
    }
}

impl Composer for SvgCollectingComposer {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        _layout: &ComputedLayout,
    ) -> crate::Result<()> {
        let svg = canvas.as_svg(content_map)?;
        self.pages.lock().unwrap()[page_idx] = svg;
        Ok(())
    }
}

pub(crate) struct PngCollectingComposer<'a> {
    pages: Mutex<Vec<Vec<u8>>>,
    resources: &'a Resources,
}

impl<'a> PngCollectingComposer<'a> {
    pub fn new(resources: &'a Resources, n_pages: usize) -> Self {
        Self {
            pages: Mutex::new(vec![Vec::new(); n_pages]),
            resources,
        }
    }

    pub fn finish(self) -> Vec<Vec<u8>> {
        self.pages.into_inner().unwrap()
    }
}

impl<'a> Composer for PngCollectingComposer<'a> {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        _layout: &ComputedLayout,
    ) -> crate::Result<()> {
        let svg = canvas.as_svg(content_map)?;
        let data = svg_to_png(self.resources, &svg)?;
        self.pages.lock().unwrap()[page_idx] = data;
        Ok(())
    }
}

fn svg_to_png(resources: &Resources, svg: &str) -> crate::Result<Vec<u8>> {
    let options = usvg::Options {
        fontdb: resources.font_db.as_ref().unwrap().clone(),
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(svg, &options)?;
    let zoom = 1.0;
    let pixmap_size = tree.size().to_int_size().scale_by(zoom).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);
    resvg::render(&tree, render_ts, &mut pixmap.as_mut());

    pixmap
        .encode_png()
        .map_err(|e| crate::Error::Generic(e.to_string()))
}
