use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Image {
    pub filename: PathBuf,
}