use crate::common::error::NelsieError;
use crate::model::ImageManager;

use svg2pdf::usvg::fontdb;

use crate::model::textstyles::FontData;
use svg2pdf::usvg::fontdb::Family::Name;
use svg2pdf::usvg::fontdb::Source;

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub(crate) struct Resources {
    pub font_db: fontdb::Database,
    pub image_manager: ImageManager,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl Resources {
    pub fn new() -> Resources {
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();
        let syntax_set = SyntaxSet::load_defaults_nonewlines();
        let theme_set = ThemeSet::load_defaults();

        Resources {
            font_db,
            image_manager: ImageManager::default(),
            syntax_set,
            theme_set,
        }
    }

    pub fn load_fonts_dir<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.font_db.load_fonts_dir(path)
    }

    pub fn check_font(&self, family_name: &str) -> crate::Result<FontData> {
        if let Some(font_id) = self.font_db.query(&fontdb::Query {
            families: &[Name(family_name)],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        }) {
            let source = self.font_db.face_source(font_id).unwrap();
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
