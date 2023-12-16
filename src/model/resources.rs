use crate::common::error::NelsieError;
use crate::model::ImageManager;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;

use usvg::fontdb;

use crate::model::textstyles::FontData;
use usvg::fontdb::Family::Name;
use usvg::fontdb::Source;

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

    pub fn load_fonts_dir<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.font_db.load_fonts_dir(path)
    }

    pub fn check_font<'a>(&mut self, family_name: &'a str) -> crate::Result<FontData> {
        if let Some(font_id) = self.font_db.query(&fontdb::Query {
            families: &[Name(family_name)],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        }) {
            let source = self.font_db.face_source(font_id).unwrap();
            let descender = match source {
                // Small code redundancy because of lifetimes
                (Source::File(file), idx) => {
                    let data = std::fs::read(file)?;
                    let face = ttf_parser::Face::parse(&data, idx)
                        .map_err(|_| NelsieError::generic_err("Failed to parse font"))?;
                    face.descender() as f32 / face.height() as f32
                }
                (Source::Binary(data), idx) => {
                    let face = ttf_parser::Face::parse(data.as_ref().as_ref(), idx)
                        .map_err(|_| NelsieError::generic_err("Failed to parse font"))?;
                    face.descender() as f32 / face.height() as f32
                }
                _ => {
                    todo!()
                }
            };
            Ok(FontData {
                family_name: family_name.to_string(),
                descender,
            })
        } else {
            Err(NelsieError::Generic(format!(
                "Font '{}' not found.",
                family_name
            )))
        }
    }
}
