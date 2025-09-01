use crate::render::content::ContentMap;

pub(crate) struct RenderContext<'a> {
    //pub resources: &'a Resources,
    pub content_map: &'a ContentMap,
}
