use crate::model::LoadedImage;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Video {
    pub path: PathBuf,
    pub cover_image: Option<Arc<LoadedImage>>,
    pub data_type: String,
    pub show_controls: bool,
}

#[derive(Debug)]
pub(crate) struct NodeContentVideo {
    pub video: Arc<Video>,
}
