use crate::render::draw::DrawItem;
use crate::{Color, ContentId, Rectangle};

#[derive(Debug)]
pub(crate) enum CanvasItem {
    Content {
        rect: Rectangle,
        content_id: ContentId,
    },
    DrawItem(DrawItem),
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
    pub(super) items: Vec<(i32, CanvasItem)>,
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

    pub fn items(&self) -> impl Iterator<Item = &CanvasItem> {
        self.items.iter().map(|item| &item.1)
    }

    pub fn finish(&mut self) {
        self.items.sort_by_key(|item| item.0);
    }

    pub fn add_content(&mut self, z_level: i32, rect: Rectangle, content_id: ContentId) {
        self.items
            .push((z_level, CanvasItem::Content { rect, content_id }))
    }

    pub fn add_draw_item(&mut self, z_level: i32, item: DrawItem) {
        self.items.push((z_level, CanvasItem::DrawItem(item)));
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }
}
