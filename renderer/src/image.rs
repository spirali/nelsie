use by_address::ByAddress;
use std::sync::Arc;

#[derive(Debug)]
pub struct ImagePlacement {
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InMemoryImage {
    Png(ByAddress<Arc<[u8]>>),
    Jpeg(ByAddress<Arc<[u8]>>),
    Svg(String),
}
