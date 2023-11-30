use crate::model::Step;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Image {
    pub filename: PathBuf,
    pub enable_steps: bool,
    pub shift_steps: Step,
}
