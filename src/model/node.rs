use super::{Color, Size, StepValue};
use serde::Deserialize;
use crate::model::text::StyledText;

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    pub children: Option<Vec<Node>>,

    pub show: StepValue<bool>,

    pub width: StepValue<Size>,
    pub height: StepValue<Size>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub bg_color: StepValue<Option<Color>>,
    pub text: StepValue<Option<StyledText>>,
}
