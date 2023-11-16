use super::{Color, Size, StepValue};
use crate::model::text::StyledText;
use crate::model::{LayoutExpr, NodeId, Step};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Node {
    pub node_id: NodeId,
    pub children: Option<Vec<Node>>,

    pub show: StepValue<bool>,

    pub x: StepValue<Option<LayoutExpr>>,
    pub y: StepValue<Option<LayoutExpr>>,

    pub width: StepValue<Size>,
    pub height: StepValue<Size>,

    pub row: StepValue<bool>,
    pub reverse: StepValue<bool>,

    pub bg_color: StepValue<Option<Color>>,
    pub text: StepValue<Option<StyledText>>,
}

impl Node {
    pub fn main_axis_position(&self, has_row_parent: bool) -> &StepValue<Option<LayoutExpr>> {
        if has_row_parent {
            &self.x
        } else {
            &self.y
        }
    }
}
