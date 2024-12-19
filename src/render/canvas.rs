use crate::common::{Color, DrawItem, Rectangle};
use crate::model::Video;
use crate::render::text::RenderedText;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) enum CanvasItem {
    PngImage {
        rect: Rectangle,
        data: Arc<Vec<u8>>,
    },
    JpegImage {
        rect: Rectangle,
        data: Arc<Vec<u8>>,
    },
    SvgImage {
        rect: Rectangle,
        data: String,
        width: f32,
        height: f32,
    },
    DrawItems(Vec<DrawItem>),
    Text {
        text: Arc<RenderedText>,
        x: f32,
        y: f32,
    },
    Video {
        rect: Rectangle,
        video: Arc<Video>,
    },
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

    pub fn add_jpeg_image(&mut self, rect: Rectangle, data: Arc<Vec<u8>>) {
        self.items.push(CanvasItem::JpegImage { rect, data });
    }

    pub fn add_video(&mut self, rect: Rectangle, video: Arc<Video>) {
        self.items.push(CanvasItem::Video { rect, video });
    }

    pub fn add_png_image(&mut self, rect: Rectangle, data: Arc<Vec<u8>>) {
        self.items.push(CanvasItem::PngImage { rect, data });
    }

    pub fn add_text(&mut self, text: Arc<RenderedText>, x: f32, y: f32) {
        self.items.push(CanvasItem::Text { text, x, y })
    }

    pub fn add_svg_image(&mut self, rect: Rectangle, data: String, width: f32, height: f32) {
        self.items.push(CanvasItem::SvgImage {
            rect,
            data,
            width,
            height,
        })
    }

    pub fn add_draw_item(&mut self, item: DrawItem) {
        if let Some(CanvasItem::DrawItems(ref mut items)) = self.items.last_mut() {
            items.push(item)
        } else {
            self.items.push(CanvasItem::DrawItems(vec![item]))
        }
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }
}
