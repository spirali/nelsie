use crate::ContentId;
use crate::render::content::{Content, ContentMap};
use crate::render::text::TextContext;
use crate::resources::Resources;
use std::collections::HashMap;

pub(crate) struct RenderContext<'a> {
    //pub resources: &'a Resources,
    pub content_map: &'a ContentMap,
}
