use parley::fontique::{Collection, CollectionOptions, SourceCache};
use parley::{FontContext, GenericFamily};
use resvg::usvg::fontdb;
use std::path::Path;
use std::sync::Arc;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct Resources {
    // // FontContext is needed for parley (normal text rendering)
    pub font_context: FontContext,
    // // FontDB is needed for rendering SVG
    // // Because we need to fontdb::Database to usvg::Options, we need to wrap it in Arc
    pub font_db: Option<Arc<fontdb::Database>>,
    // pub image_manager: ImageManager,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl Resources {
    pub fn new(
        system_fonts: bool,
        system_fonts_for_svg: bool,
        default_code_syntaxes: bool,
        default_code_themes: bool,
    ) -> Resources {
        let mut font_db = fontdb::Database::new();
        if system_fonts || system_fonts_for_svg {
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
            font_context: FontContext {
                collection: Collection::new(CollectionOptions {
                    shared: true,
                    system_fonts,
                }),
                source_cache: SourceCache::new_shared(),
            },
            font_db: Some(Arc::new(font_db)),
            syntax_set,
            theme_set,
        }
    }

    pub fn set_generic_family(&mut self, name: &str, font_name: &str) -> crate::Result<()> {
        let family = GenericFamily::parse(name)
            .ok_or_else(|| crate::Error::generic_err(format!("Invalid generic family '{name}'")))?;
        let font_id = self
            .font_context
            .collection
            .family_id(font_name)
            .ok_or_else(|| crate::Error::generic_err(format!("Font '{font_name}' not found")))?;
        self.font_context
            .collection
            .set_generic_families(family, [font_id].into_iter());
        Ok(())
    }

    pub fn load_code_syntax_dir(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Adding code syntax directory {}", path.display());
        let syntax_set = std::mem::take(&mut self.syntax_set);
        let mut builder = syntax_set.into_builder();
        builder
            .add_from_folder(path, false)
            .map_err(|e| crate::Error::Generic(format!("Adding syntax failed: {}", e)))?;
        self.syntax_set = builder.build();
        Ok(())
    }

    pub fn load_code_theme_dir(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Adding code theme directory {}", path.display());
        self.theme_set
            .add_from_folder(path)
            .map_err(|e| crate::Error::Generic(format!("Adding theme failed: {}", e)))?;
        Ok(())
    }

    pub fn load_fonts_dir(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Adding font directory {}", path.display());
        let paths = std::fs::read_dir(path)?;
        for entry in paths {
            let entry = entry?;
            if !entry.metadata()?.is_file() {
                continue;
            }
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| !["ttf", "otf", "ttc", "otc"].contains(&ext))
                .unwrap_or(true)
            {
                continue;
            }
            log::debug!("Loading font {}", path.display());
            let font_data = std::fs::read(&path)?;
            self.font_context.collection.register_fonts(font_data);
        }
        let font_db = std::mem::take(&mut self.font_db).unwrap();
        let mut font_db = Arc::unwrap_or_clone(font_db);
        font_db.load_fonts_dir(path);
        self.font_db = Some(Arc::new(font_db));
        Ok(())
    }

    pub fn check_font(&mut self, family_name: &str) -> bool {
        GenericFamily::parse(family_name).is_some()
            || self
                .font_context
                .collection
                .family_id(family_name)
                .is_some()
    }
}
