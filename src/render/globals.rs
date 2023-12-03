use std::collections::HashMap;
use std::path::{Path, PathBuf};
use usvg::fontdb;

pub(crate) struct GlobalResources {
    font_db: fontdb::Database,
}

impl GlobalResources {
    pub fn new(font_db: fontdb::Database) -> Self {
        log::debug!("Loading system font database");
        GlobalResources {
            font_db,
        }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

}
