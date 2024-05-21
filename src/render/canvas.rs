use crate::common::Rectangle;
use crate::model::Color;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) enum CanvasItem {
    SvgChunk(String),
    PngImage(Rectangle, Arc<Vec<u8>>),
    JpegImage(Rectangle, Arc<Vec<u8>>),
    SvgImage(Rectangle, String, f32, f32),
}

#[derive(Debug)]
pub(crate) struct Link {
    rect: Rectangle,
    url: String,
}

impl Link {
    pub fn new(rect: Rectangle, url: String) -> Self {
        Link { rect, url }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn rect(&self) -> &Rectangle {
        &self.rect
    }
}

#[derive(Debug)]
pub(crate) struct Canvas {
    pub(super) items: Vec<CanvasItem>,
    pub(super) links: Vec<Link>,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) bg_color: Color,
}

impl Canvas {
    pub fn new(width: f32, height: f32, bg_color: Color) -> Self {
        Self {
            width,
            height,
            bg_color,
            items: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: CanvasItem) {
        self.items.push(item)
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }
}
