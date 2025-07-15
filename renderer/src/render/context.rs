use crate::render::text::TextContext;
use crate::resources::Resources;

pub(crate) struct RenderContext<'a> {
    pub resources: &'a Resources,
    pub thread_resources: &'a mut ThreadLocalResources,
}

pub(crate) struct ThreadLocalResources {
    pub text_context: TextContext,
}
