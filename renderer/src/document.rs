use crate::NodeChild::Node;
use crate::image::InMemoryImage;
use crate::render::composer::{
    Composer, PngCollectorComposer, PngWriteComposer, SvgCollectorComposer, SvgWriteComposer,
};
use crate::render::composer_pdf::PdfComposer;
use crate::render::context::{RenderContext, ThreadLocalResources};
use crate::render::text::{RenderedText, TextContext};
use crate::resources::Resources;
use crate::text::{Text, TextId};
use crate::{Color, ImageId, NodeId, Page};
use itertools::Itertools;
use miniz_oxide::deflate::CompressionLevel;
use parley::{FontContext, LayoutContext};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Register {
    node_id_counter: NodeId,
    texts: HashMap<Text, (TextId, u32)>,
    images_paths: HashMap<PathBuf, ImageId>,
    images_mem: HashMap<InMemoryImage, ImageId>,
    image_id_counter: ImageId,
}

impl Register {
    pub fn new() -> Self {
        Self {
            node_id_counter: NodeId::new(0),
            texts: HashMap::new(),
            images_paths: HashMap::default(),
            images_mem: HashMap::default(),
            image_id_counter: ImageId::new(0),
        }
    }

    #[inline]
    pub fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter.bump()
    }

    pub fn register_text(&mut self, text: Text) -> TextId {
        let count = self.texts.len();
        let entry = self
            .texts
            .entry(text)
            .or_insert_with(|| (TextId::new(count as u32), 0));
        entry.1 += 1;
        entry.0
    }

    pub fn register_image_path(&mut self, path: &std::path::Path) -> ImageId {
        if let Some(image_id) = self.images_paths.get(path) {
            return *image_id;
        }
        let image_id = self.image_id_counter.bump();
        self.images_paths.insert(path.to_path_buf(), image_id);
        image_id
    }

    pub fn register_image_mem(&mut self, image: InMemoryImage) -> ImageId {
        let entry = self
            .images_mem
            .entry(image)
            .or_insert_with(|| self.image_id_counter.bump());
        *entry
    }
}

pub struct Document {
    pages: Vec<Page>,
    register: Register,
}

enum PreprocessingJob<'a> {
    Text(&'a Text, TextId),
    ImageMem(&'a InMemoryImage, ImageId),
    ImagePath(&'a std::path::Path, ImageId),
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
        composer: &dyn Composer,
    ) -> crate::Result<()> {
        let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
        if let Some(n_threads) = options.n_threads {
            thread_pool_builder = thread_pool_builder.num_threads(n_threads);
        }
        let thread_pool = thread_pool_builder.build().unwrap();
        thread_pool.install(|| {
            let (text_cache, image_cache): (
                crate::Result<HashMap<_, _>>,
                crate::Result<HashMap<_, _>>,
            ) = rayon::join(
                || {
                    self.register
                        .texts
                        .iter()
                        .collect_vec()
                        .into_par_iter()
                        .map_init(
                            || FontContext {
                                collection: resources.font_context.collection.clone(),
                                source_cache: resources.font_context.source_cache.clone(),
                            },
                            |font_ctx, (text, (node_id, count))| todo!(),
                        )
                        .collect::<crate::Result<HashMap<NodeId, RenderedText>>>()
                },
                || {
                    //self.images_paths.par_iter()
                    // todo!()
                    Ok(HashMap::new())
                },
            );
            let text_cache = text_cache?;
            let mut image_cache: HashMap<ImageId, InMemoryImage> = image_cache?;

            for (image, image_id) in self.register.images_mem.iter() {
                assert!(
                    image_cache
                        .insert(image_id.clone(), image.clone())
                        .is_none()
                );
            }

            self.pages.par_iter().enumerate().try_for_each_init(
                || ThreadLocalResources {
                    text_context: TextContext {
                        layout_cx: LayoutContext::new(),
                        font_cx: FontContext {
                            collection: resources.font_context.collection.clone(),
                            source_cache: resources.font_context.source_cache.clone(),
                        },
                    },
                },
                |thread_resources, (page_idx, page)| {
                    let mut render_ctx = RenderContext {
                        resources,
                        thread_resources,
                    };
                    let canvas = page.render_to_canvas(&mut render_ctx);
                    composer.add_page(page_idx, canvas)
                },
            )
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
        let composer = PdfComposer::new(self.pages.len(), options.compression_level);
        self.render(resources, options, &composer)?;
        Ok(composer.finish())
    }

    pub fn render_svg_to_dir(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        std::fs::create_dir_all(path)?;
        let composer = SvgWriteComposer::new(path, self.pages.len());
        self.render(resources, options, &composer)
    }

    pub fn render_png_to_dir(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        std::fs::create_dir_all(path)?;
        let composer = PngWriteComposer::new(resources, path, self.pages.len());
        self.render(resources, options, &composer)
    }

    pub fn render_svg_to_vec(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<String>> {
        let composer = SvgCollectorComposer::new(self.pages.len());
        self.render(resources, options, &composer)?;
        Ok(composer.finish())
    }

    pub fn render_png_to_vec(
        &self,
        resources: &Resources,
        options: &RenderingOptions,
    ) -> crate::Result<Vec<Vec<u8>>> {
        let composer = PngCollectorComposer::new(resources, self.pages.len());
        self.render(resources, options, &composer)?;
        Ok(composer.finish())
    }
}
