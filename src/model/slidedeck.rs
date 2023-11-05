use super::node::Node;
use serde::Deserialize;
use crate::model::Step;

#[derive(Debug, Deserialize)]
pub(crate) struct Slide {
    pub width: f32,
    pub height: f32,
    pub node: Node,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SlideDeck {
    pub slides: Vec<Slide>,
}

impl Slide {
    pub fn n_steps(&self) -> Step {
        1
    }
}