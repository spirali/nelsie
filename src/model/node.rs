use super::{Color, Size, StepValue};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    #[serde(default)]
    pub children: Vec<Node>,

    pub show: StepValue<bool>,

    pub width: StepValue<Size>,
    pub height: StepValue<Size>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub bg_color: StepValue<Option<Color>>,
    pub text: Option<String>,
}
