use crate::node::Node;
use crate::render::composer::{
    Composer, PngCollectorComposer, PngWriteComposer, SvgCollectorComposer, SvgWriteComposer,
};
use crate::render::context::{RenderContext, ThreadLocalResources};
use crate::render::text::{RenderedText, TextContext};
use crate::resources::Resources;
use crate::text::{Text, TextId};
use crate::{Color, ImageId, NodeId, Page};
use parley::{FontContext, LayoutContext};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::image::InMemoryImage;

pub struct Document {
    pages: Vec<Page>,
    texts: HashMap<Text, (TextId, u32)>,

    images_paths: HashMap<PathBuf, ImageId>,
    images_mem: HashMap<InMemoryImage, ImageId>,
    image_id_counter: ImageId,
}

impl Document {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            texts: HashMap::new(),
            images_paths: HashMap::default(),
            images_mem: HashMap::default(),
            image_id_counter: ImageId::new(0),
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
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

    pub fn register_image_path(&mut self, path: &Path) -> ImageId {
        if let Some(image_id) = self.images_paths.get(path) {
            return *image_id;
        }
        let image_id = self.image_id_counter.bump();
        self.images_paths.insert(path.to_path_buf(), image_id);
        image_id
    }
    
    fn render(&self, resources: &Resources, composer: &dyn Composer) -> crate::Result<()> {
        let text_cache: HashMap<_, _> = self
            .texts
            .par_iter()
            .map_init(
                || FontContext {
                    collection: resources.font_context.collection.clone(),
                    source_cache: resources.font_context.source_cache.clone(),
                },
                |font_ctx, (text, (node_id, count))| todo!(),
            )
            .collect::<crate::Result<HashMap<NodeId, RenderedText>>>()?;

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
                composer.add_page(page_idx, &canvas)
            },
        )
    }

    pub fn render_svg_to_dir(
        &self,
        resources: &Resources,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        let composer = SvgWriteComposer::new(path, self.pages.len());
        self.render(resources, &composer)
    }

    pub fn render_png_to_dir(
        &self,
        resources: &Resources,
        path: &std::path::Path,
    ) -> crate::Result<()> {
        let composer = PngWriteComposer::new(path, self.pages.len());
        self.render(resources, &composer)
    }

    pub fn render_svg_to_vec(&self, resources: &Resources) -> crate::Result<Vec<String>> {
        let composer = SvgCollectorComposer::new(self.pages.len());
        self.render(resources, &composer)?;
        Ok(composer.finish())
    }

    pub fn render_png_to_vec(&self, resources: &Resources) -> crate::Result<Vec<Vec<u8>>> {
        let composer = PngCollectorComposer::new(resources, self.pages.len());
        self.render(resources, &composer)?;
        Ok(composer.finish())
    }
}
