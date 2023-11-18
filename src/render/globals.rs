use std::collections::HashMap;
use std::path::{Path, PathBuf};
use usvg::fontdb;
use crate::render::image::LoadedImage;


pub(crate) struct GlobalResources {
    font_db: fontdb::Database,
    loaded_images: HashMap<PathBuf, LoadedImage>,
}


impl GlobalResources {
    pub fn new(loaded_images: HashMap<PathBuf, LoadedImage>) -> Self {
        log::debug!("Loading system font database");
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();
        GlobalResources { font_db, loaded_images }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

    pub fn get_image(&self, path: &Path) -> Option<&LoadedImage> {
        self.loaded_images.get(path)
    }
}
