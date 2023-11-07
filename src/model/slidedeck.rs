use super::node::Node;
use crate::model::Step;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Slide {
    pub width: f32,
    pub height: f32,
    pub node: Node,
    pub n_steps: Step,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SlideDeck {
    pub slides: Vec<Slide>,
}
