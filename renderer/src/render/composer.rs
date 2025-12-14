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
