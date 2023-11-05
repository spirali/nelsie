use usvg::fontdb;

pub(crate) struct GlobalResources {
    font_db: fontdb::Database,
}

impl GlobalResources {
    pub fn new() -> Self {
        log::debug!("Loading system font database");
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();
        GlobalResources { font_db }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }
}
