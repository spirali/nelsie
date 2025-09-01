use crate::layout_info::{LayoutInfoComposer, PageLayout};
use crate::node::ContentId;
use crate::render::composer::{
    Composer, PngCollectingComposer, PngWriteComposer, SvgCollectingComposer, SvgWriteComposer,
};
use crate::render::composer_pdf::PdfComposer;
use crate::render::content::{Content, ContentBody};
use crate::render::context::RenderContext;
use crate::render::layout::compute_page_layout;
use crate::render::text::{TextContext, render_text};
use crate::resources::Resources;
use crate::text::Text;
use crate::utils::fileutils::ensure_directory;
use crate::{InMemoryBinImage, InMemorySvgImage, Page, Rectangle};
use itertools::Itertools;
use parley::FontContext;
use pdf_writer::Finish;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Composition {
    content_id: ContentId,
    width: f32,
    height: f32,
    items: Vec<(Rectangle, ContentId)>,
}

pub struct Register {
    content_id_counter: ContentId,
    texts: HashMap<Text, (ContentId, u32)>,
    bin_images: HashMap<InMemoryBinImage, (ContentId, f32, f32)>,
    svg_images: HashMap<InMemorySvgImage, (ContentId, usize, f32, f32)>,
    compositions: Vec<Composition>,
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            content_id_counter: ContentId::new(0),
            texts: HashMap::new(),
            bin_images: HashMap::new(),
            svg_images: HashMap::new(),
            compositions: Vec::new(),
        }
    }

    pub fn register_text(&mut self, text: Text) -> ContentId {
        let entry = self
            .texts
            .entry(text)
            .or_insert_with(|| (self.content_id_counter.bump(), 0));
        entry.1 += 1;
        entry.0
    }

    pub fn register_bin_image(
        &mut self,
        image: InMemoryBinImage,
        width: f32,
        height: f32,
    ) -> ContentId {
        let entry = self
            .bin_images
            .entry(image)
            .or_insert_with(|| (self.content_id_counter.bump(), width, height));
        entry.0
    }

    pub fn register_svg_image(
        &mut self,
        image: InMemorySvgImage,
        width: f32,
        height: f32,
    ) -> ContentId {
        let entry = self
            .svg_images
            .entry(image)
            .or_insert_with(|| (self.content_id_counter.bump(), 0, width, height));
        entry.1 += 1;
        entry.0
    }

    pub fn register_composition(
        &mut self,
        width: f32,
        height: f32,
        items: Vec<(Rectangle, ContentId)>,
    ) -> ContentId {
        let content_id = self.content_id_counter.bump();
        self.compositions.push(Composition {
            content_id,
            width,
            height,
            items,
        });
        content_id
    }
}

pub struct Document {
    pages: Vec<Page>,
    register: Register,
}

pub struct RenderingOptions {
    pub compression_level: u8,
    pub n_threads: Option<usize>,
    pub progressbar: bool,
}

impl Document {
    pub fn new(pages: Vec<Page>, register: Register) -> Self {
        Self { pages, register }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    fn render(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        composer: &mut dyn Composer,
    ) -> crate::Result<()> {
        let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
        if let Some(n_threads) = options.n_threads {
            thread_pool_builder = thread_pool_builder.num_threads(n_threads);
        }
        let thread_pool = thread_pool_builder.build().unwrap();
        let progressbar = options.progressbar.then(|| {
            let total = self.register.texts.len()
                + self.register.bin_images.len()
                + self.register.svg_images.len()
                + self.pages.len();
            indicatif::ProgressBar::new(total as u64)
        });
        thread_pool.install(|| {
            let (texts, images) = rayon::join(
                || {
                    self.register
                        .texts
                        .iter()
                        .collect_vec()
                        .into_par_iter()
                        .map_init(
                            || TextContext {
                                layout_cx: Default::default(),
                                font_cx: FontContext {
                                    collection: resources.font_context.collection.clone(),
                                    source_cache: resources.font_context.source_cache.clone(),
                                },
                            },
                            |text_ctx, (text, (content_id, count))| {
                                let (rtext, width, height) =
                                    render_text(resources, text_ctx, text)?;
                                let content = Content::new(
                                    width,
                                    height,
                                    ContentBody::Text((Arc::new(rtext), *count > 1)),
                                );
                                composer.preprocess_content(resources, *content_id, &content)?;
                                if let Some(p) = &progressbar {
                                    p.inc(1);
                                }
                                Ok((*content_id, content))
                            },
                        )
                        .collect::<crate::Result<Vec<_>>>()
                },
                || {
                    let image_contents = self
                        .register
                        .bin_images
                        .iter()
                        .map(|(img, (content_id, width, height))| {
                            (
                                *content_id,
                                Content::new(*width, *height, ContentBody::BinImage(img.clone())),
                            )
                        })
                        .chain(self.register.svg_images.iter().map(
                            |(img, (content_id, _, width, height))| {
                                (
                                    *content_id,
                                    Content::new(
                                        *width,
                                        *height,
                                        ContentBody::SvgImage(img.clone()),
                                    ),
                                )
                            },
                        ))
                        .collect_vec();
                    if composer.needs_image_preprocessing() {
                        image_contents
                            .par_iter()
                            .try_for_each(|(content_id, content)| {
                                let r =
                                    composer.preprocess_content(resources, *content_id, content);
                                if let Some(p) = &progressbar {
                                    p.inc(1);
                                }
                                r
                            })?;
                    } else if let Some(p) = &progressbar {
                        p.inc(image_contents.len() as u64);
                    }

                    crate::Result::Ok(image_contents)
                },
            );
            let texts = texts?;
            let images = images?;
            let content_map: HashMap<_, _> = texts
                .into_iter()
                .chain(images)
                .chain(self.register.compositions.iter().map(
                    |Composition {
                         content_id,
                         width,
                         height,
                         items,
                     }| {
                        (
                            *content_id,
                            Content::new(*width, *height, ContentBody::Composition(items.clone())),
                        )
                    },
                ))
                .collect();

            composer.preprocessing_finished();

            let r = self
                .pages
                .par_iter()
                .enumerate()
                .try_for_each(|(page_idx, page)| {
                    let mut render_ctx = RenderContext {
                        content_map: &content_map,
                    };
                    let layout = compute_page_layout(&mut render_ctx, page);
                    let canvas = page.render_to_canvas(&layout);
                    let r = composer.add_page(page_idx, canvas, render_ctx.content_map, &layout);
                    if let Some(p) = &progressbar {
                        p.inc(1);
                    }
                    r
                });
            if let Some(p) = progressbar {
                p.finish();
            }
            r
        })
    }

    pub fn render_pdf_to_file(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        let data = self.render_pdf_to_mem(resources, options)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn render_pdf_to_mem(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<u8>> {
        let mut composer = PdfComposer::new(self.pages.len(), options.compression_level);
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }

    pub fn render_svg_to_dir(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        ensure_directory(path)?;
        let mut composer = SvgWriteComposer::new(path, self.pages.len());
        self.render(resources, options, &mut composer)
    }

    pub fn render_png_to_dir(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        ensure_directory(path)?;
        let mut composer = PngWriteComposer::new(resources, path, self.pages.len());
        self.render(resources, options, &mut composer)
    }

    pub fn render_svg_to_vec(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<String>> {
        let mut composer = SvgCollectingComposer::new(self.pages.len());
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }

    pub fn render_png_to_vec(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<Vec<u8>>> {
        let mut composer = PngCollectingComposer::new(resources, self.pages.len());
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }

    pub fn render_layout_info(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<PageLayout>> {
        let mut composer = LayoutInfoComposer::new(self.pages.len());
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }
}
