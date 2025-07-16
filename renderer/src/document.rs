use std::io::Bytes;
use crate::node::Node;
use crate::render::composer::{Composer, PngCollectorComposer, PngWriteComposer, SvgCollectorComposer, SvgWriteComposer};
use crate::render::context::{RenderContext, ThreadLocalResources};
use crate::render::text::TextContext;
use crate::resources::Resources;
use crate::{Color, Page};
use parley::{FontContext, LayoutContext};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Document {
    pub pages: Vec<Page>,
}

impl Document {
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    fn render(&self, resources: &Resources, composer: &dyn Composer) -> crate::Result<()> {
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
