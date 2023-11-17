use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use usvg::fontdb;
use crate::NelsieError;

pub(crate) enum LoadedImageData {
    Png(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    Jpeg(Arc<Vec<u8>>),
}

pub(crate) struct LoadedImage {
    pub width: f32,
    pub height: f32,
    pub data: LoadedImageData,
}


pub(crate) struct GlobalResources {
    font_db: fontdb::Database,
    loaded_images: HashMap<PathBuf, LoadedImage>,
}

fn load_raster_image(raw_data: Vec<u8>) -> Option<LoadedImage> {
    let size = imagesize::blob_size(&raw_data).ok()?;
    let image_type = imagesize::image_type(&raw_data);
    let data_arc = Arc::new(raw_data);
    let data = match image_type {
        Ok(imagesize::ImageType::Png) => LoadedImageData::Png(data_arc),
        Ok(imagesize::ImageType::Jpeg) => LoadedImageData::Jpeg(data_arc),
        Ok(imagesize::ImageType::Gif) => LoadedImageData::Gif(data_arc),
        _ => unreachable!() // This is safe, otherwise it should already fail in blob_size
    };
    Some(LoadedImage {
        width: size.width as f32,
        height: size.height as f32,
        data,
    })
}

impl GlobalResources {
    pub fn new() -> Self {
        log::debug!("Loading system font database");
        let mut font_db = fontdb::Database::new();
        font_db.load_system_fonts();
        GlobalResources { font_db, loaded_images: HashMap::new() }
    }

    pub fn font_db(&self) -> &fontdb::Database {
        &self.font_db
    }

    pub fn load_image(&mut self, path: &Path) -> crate::Result<()> {
        log::debug!("Loading image: {}", path.display());

        let raw_data = std::fs::read(path)?;
        let image = load_raster_image(raw_data).ok_or_else(|| NelsieError::GenericError(format!("Image '{}' has unknown format", path.display())))?;
        assert!(self.loaded_images.insert(path.to_path_buf(), image).is_none());

        Ok(())
    }

    pub fn get_image(&self, path: &Path) -> Option<&LoadedImage> {
        self.loaded_images.get(path)
    }
}
