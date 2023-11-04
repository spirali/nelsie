use super::node::Node;
use serde::Deserialize;


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