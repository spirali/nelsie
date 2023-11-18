use crate::render::image::LoadedImage;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use usvg::fontdb;

pub(crate) struct GlobalResources {
    font_db: fontdb::Database,
    loaded_images: HashMap<PathBuf, LoadedImage>,
}

impl GlobalResources {
    pub fn new(font_db: fontdb::Database, loaded_images: HashMap<PathBuf, LoadedImage>) -> Self {
        log::debug!("Loading system font database");
        GlobalResources {
            font_db,
            loaded_images,
        }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

    pub fn get_image(&self, path: &Path) -> Option<&LoadedImage> {
        self.loaded_images.get(path)
    }
}
