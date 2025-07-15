use crate::render::text::RenderedText;
use crate::{ContentId, InMemoryBinImage, InMemorySvgImage, Rectangle};
use std::collections::HashMap;

pub(crate) type ContentMap = HashMap<ContentId, Content>;

pub(crate) struct Content {
    width: f32,
    height: f32,
    body: ContentBody,
}

impl Content {
    pub fn new(width: f32, height: f32, body: ContentBody) -> Self {
        Content {
            width,
            height,
            body,
        }
    }

    #[inline]
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn body(&self) -> &ContentBody {
        &self.body
    }
}

pub(crate) enum ContentBody {
    Text((RenderedText, bool)),
    BinImage(InMemoryBinImage),
    SvgImage(InMemorySvgImage),
    Composition(Vec<(Rectangle, ContentId)>),
}
