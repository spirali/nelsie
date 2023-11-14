use super::{Color, Size, StepValue};
use crate::model::text::{StyledText};
use serde::Deserialize;
use crate::model::{NodeId, PosAndSizeExpr};

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    pub node_id: NodeId,
    pub children: Option<Vec<Node>>,

    pub show: StepValue<bool>,

    // pub x: StepValue<Option<PosAndSizeExpr>>,
    // pub y: StepValue<Option<PosAndSizeExpr>>,

    pub width: StepValue<Size>,
    pub height: StepValue<Size>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub bg_color: StepValue<Option<Color>>,
    pub text: StepValue<Option<StyledText>>,
}
