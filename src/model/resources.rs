use crate::common::error::NelsieError;
use crate::model::ImageManager;

use usvg::fontdb;

use usvg::fontdb::Family::Name;

pub(crate) struct Resources {
    pub font_db: fontdb::Database,
    pub image_manager: ImageManager,
}

impl Resources {
    pub fn new() -> Resources {
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();
        Resources {
            font_db,
            image_manager: ImageManager::default(),
        }
    }

    pub fn has_font(&self, family_name: &str) -> bool {
        self.font_db
            .query(&fontdb::Query {
                families: &[Name(family_name)],
                weight: Default::default(),
                stretch: Default::default(),
                style: Default::default(),
            })
            .is_some()
    }

    pub fn check_font<'a>(&self, family_name: &'a str) -> crate::Result<&'a str> {
        if !self.has_font(family_name) {
            return Err(NelsieError::Generic(format!(
                "Font '{}' not found.",
                family_name
            )));
        }
        Ok(family_name)
    }
}
