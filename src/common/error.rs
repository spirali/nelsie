use crate::common::fileutils::ensure_directory;
use crate::model::{Slide, SlideDeck};
use crate::render::{
    check_fonts, load_image_in_deck, render_slide_step, GlobalResources, PdfBuilder, RenderConfig,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use thiserror::Error;
use usvg::fontdb;

#[derive(Debug, Error)]
pub enum NelsieError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    XmlError(#[from] roxmltree::Error),
    #[error(transparent)]
    SvgError(#[from] usvg::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Error: {0}")]
    GenericError(String),
}

pub type Result<T> = std::result::Result<T, NelsieError>;
