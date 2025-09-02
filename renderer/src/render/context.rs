use crate::render::content::ContentMap;

pub(crate) struct RenderContext<'a> {
    pub content_map: &'a ContentMap,
}
