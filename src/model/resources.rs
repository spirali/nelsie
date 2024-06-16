use crate::common::error::NelsieError;
use crate::model::ImageManager;
use std::path::Path;
use std::sync::Arc;

use svg2pdf::usvg::fontdb;

use crate::model::textstyles::FontData;
use svg2pdf::usvg::fontdb::Family::Name;
use svg2pdf::usvg::fontdb::Source;

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub(crate) struct Resources {
    // Because we need to fontdb::Database to usvg::Options, we need to wrap it in Arc
    pub font_db: Option<Arc<fontdb::Database>>,
    pub image_manager: ImageManager,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl Resources {
    pub fn new(
        system_fonts: bool,
        default_code_syntaxes: bool,
        default_code_themes: bool,
    ) -> Resources {
        let mut font_db = fontdb::Database::new();
        if system_fonts {
            font_db.load_system_fonts();
        }
        let syntax_set = if default_code_syntaxes {
            SyntaxSet::load_defaults_nonewlines()
        } else {
            SyntaxSet::new()
        };

        let theme_set = if default_code_themes {
            ThemeSet::load_defaults()
        } else {
            ThemeSet::new()
        };

        Resources {
            font_db: Some(Arc::new(font_db)),
            image_manager: ImageManager::default(),
            syntax_set,
            theme_set,
        }
    }

    pub fn font_db_arc(&self) -> Arc<fontdb::Database> {
        self.font_db.as_ref().unwrap().clone()
    }

    pub fn load_code_syntax_dir(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Adding code syntax directory {}", path.display());
        let syntax_set = std::mem::take(&mut self.syntax_set);
        let mut builder = syntax_set.into_builder();
        builder
            .add_from_folder(path, false)
            .map_err(|e| NelsieError::Generic(format!("Adding syntax failed: {}", e)))?;
        self.syntax_set = builder.build();
        Ok(())
    }

    pub fn load_code_theme_dir(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Adding code theme directory {}", path.display());
        self.theme_set
            .add_from_folder(path)
            .map_err(|e| NelsieError::Generic(format!("Adding theme failed: {}", e)))?;
        Ok(())
    }

    pub fn load_fonts_dir(&mut self, path: &Path) {
        log::debug!("Adding font directory {}", path.display());
        let font_db = std::mem::take(&mut self.font_db).unwrap();
        let mut font_db = Arc::unwrap_or_clone(font_db);
        font_db.load_fonts_dir(path);
        self.font_db = Some(Arc::new(font_db));
    }

    pub fn check_font(&self, family_name: &str) -> crate::Result<FontData> {
        if let Some(font_id) = self.font_db.as_ref().unwrap().query(&fontdb::Query {
            families: &[Name(family_name)],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        }) {
            let source = self.font_db.as_ref().unwrap().face_source(font_id).unwrap();
            let (descender, space_size) = match source {
                // Small code redundancy because of lifetimes
                (Source::File(file), idx) => {
                    let data = std::fs::read(file)?;
                    let face = ttf_parser::Face::parse(&data, idx)
                        .map_err(|_| NelsieError::generic_err("Failed to parse font"))?;
                    let glyph_id = face.glyph_index(' ').unwrap();
                    let size = face.height() as f32;
                    (
                        face.descender() as f32 / size,
                        face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32
                            / face.units_per_em() as f32,
                    )
                }
                (Source::Binary(data), idx) => {
                    let face = ttf_parser::Face::parse(data.as_ref().as_ref(), idx)
                        .map_err(|_| NelsieError::generic_err("Failed to parse font"))?;
                    let glyph_id = face.glyph_index(' ').unwrap();
                    let size = face.height() as f32;
                    (
                        face.descender() as f32 / size,
                        face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32 / size,
                    )
                }
                _ => {
                    todo!()
                }
            };
            Ok(FontData {
                family_name: family_name.to_string(),
                descender,
                space_size,
            })
        } else {
            Err(NelsieError::Generic(format!(
                "Font '{}' not found.",
                family_name
            )))
        }
    }
}
