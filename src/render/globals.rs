use std::collections::HashMap;
use std::path::{Path, PathBuf};
use usvg::fontdb;

pub(crate) struct Resources {
    font_db: fontdb::Database,
}

impl Resources {
    pub fn new(font_db: fontdb::Database) -> Self {
        log::debug!("Loading system font database");
        Resources { font_db }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }
}
