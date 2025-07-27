use crate::NodeChild::Node;
use crate::image::InMemoryImage;
use crate::node::ContentId;
use crate::render::composer::{
    Composer, PngCollectorComposer, PngWriteComposer, SvgCollectorComposer, SvgWriteComposer,
};
use crate::render::composer_pdf::PdfComposer;
use crate::render::content::{Content, ContentBody, ContentMap};
use crate::render::context::RenderContext;
use crate::render::text::{RenderedText, TextContext, render_text};
use crate::resources::Resources;
use crate::text::Text;
use crate::utils::fileutils::ensure_directory;
use crate::{Color, NodeId, Page};
use itertools::Itertools;
use miniz_oxide::deflate::CompressionLevel;
use parley::{FontContext, LayoutContext};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Register {
    node_id_counter: NodeId,
    content_id_counter: ContentId,
    texts: HashMap<Text, (ContentId, u32)>,
    images_paths: HashMap<PathBuf, ContentId>,
    images_mem: HashMap<InMemoryImage, ContentId>,
}

impl Register {
    pub fn new() -> Self {
        Self {
            node_id_counter: NodeId::new(0),
            content_id_counter: ContentId::new(0),
            texts: HashMap::new(),
            images_paths: HashMap::default(),
            images_mem: HashMap::default(),
        }
    }

    #[inline]
    pub fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter.bump()
    }

    pub fn register_text(&mut self, text: Text) -> ContentId {
        let entry = self
            .texts
            .entry(text)
            .or_insert_with(|| (self.content_id_counter.bump(), 0));
        entry.1 += 1;
        entry.0
    }

    pub fn register_image_path(&mut self, path: &std::path::Path) -> ContentId {
        if let Some(image_id) = self.images_paths.get(path) {
            return *image_id;
        }
        let image_id = self.content_id_counter.bump();
        self.images_paths.insert(path.to_path_buf(), image_id);
        image_id
    }

    pub fn register_image_mem(&mut self, image: InMemoryImage) -> ContentId {
        let entry = self
            .images_mem
            .entry(image)
            .or_insert_with(|| self.content_id_counter.bump());
        *entry
    }
}

pub struct Document {
    pages: Vec<Page>,
    register: Register,
}

enum PreprocessingJob<'a> {
    Text(&'a Text),
    ImageMem(&'a InMemoryImage),
    ImagePath(&'a std::path::Path),
}

pub struct RenderingOptions {
    pub compression_level: u8,
    pub n_threads: Option<usize>,
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
        let content_map: Mutex<ContentMap> = Mutex::new(HashMap::new());
        thread_pool.install(|| {
            let (texts, _) = rayon::join(
                || {
                    self.register
                        .texts
                        .iter()
                        .collect_vec()
                        .into_par_iter()
                        .try_for_each_init(
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
                                    ContentBody::Text((rtext, *count > 1)),
                                );
                                composer.preprocess_content(*content_id, &content)?;
                                content_map.lock().unwrap().insert(*content_id, content);
                                crate::Result::Ok(())
                            },
                        )
                },
                || {
                    //self.images_paths.par_iter()
                    // todo!()
                    ()
                },
            );
            texts?;
            let content_map = content_map.into_inner().unwrap();

            composer.preprocessing_finished();

            self.pages
                .par_iter()
                .enumerate()
                .try_for_each(|(page_idx, page)| {
                    let mut render_ctx = RenderContext {
                        content_map: &content_map,
                    };
                    let canvas = page.render_to_canvas(&mut render_ctx);
                    composer.add_page(page_idx, canvas, &render_ctx.content_map)
                })
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
        let mut composer = SvgCollectorComposer::new(self.pages.len());
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }

    pub fn render_png_to_vec(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<Vec<u8>>> {
        let mut composer = PngCollectorComposer::new(resources, self.pages.len());
        self.render(resources, options, &mut composer)?;
        Ok(composer.finish())
    }
}
