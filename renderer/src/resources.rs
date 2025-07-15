use parley::FontContext;
use resvg::usvg::fontdb;
use std::sync::Arc;

pub(crate) struct Resources {
    // // FontContext is needed for parley (normal text rendering)
    pub font_context: FontContext,
    // // FontDB is needed for rendering SVG
    // // Because we need to fontdb::Database to usvg::Options, we need to wrap it in Arc
    pub font_db: Option<Arc<fontdb::Database>>,
    // pub image_manager: ImageManager,
    // pub syntax_set: SyntaxSet,
    // pub theme_set: ThemeSet,
}
