use crate::model::Step;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub(crate) struct Image {
    pub filename: PathBuf,
    pub enable_steps: bool,
    pub shift_steps: Step,
}
