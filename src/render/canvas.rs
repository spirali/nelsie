use crate::common::Rectangle;
use crate::model::Color;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) enum CanvasItem {
    SvgChunk(String),
    PngImage(Rectangle, Arc<Vec<u8>>),
    GifImage(Rectangle, Arc<Vec<u8>>),
    JpegImage(Rectangle, Arc<Vec<u8>>),
    SvgImage(Rectangle, String, f32, f32),
}

#[derive(Debug)]
pub(crate) struct Canvas {
    pub(super) items: Vec<CanvasItem>,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) bg_color: Color,
}

impl Canvas {
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn new(width: f32, height: f32, bg_color: Color) -> Self {
        Self {
            width,
            height,
            bg_color,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: CanvasItem) {
        self.items.push(item)
    }
}
