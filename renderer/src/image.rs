use by_address::ByAddress;
use std::sync::Arc;

// #[derive(Debug)]
// pub struct ImagePlacement {
//     pub offset_x: f32,
//     pub offset_y: f32,
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InMemoryBinImage {
    Png(ByAddress<Arc<Vec<u8>>>),
    Jpeg(ByAddress<Arc<Vec<u8>>>),
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InMemorySvgImage(ByAddress<Arc<String>>);

impl InMemoryBinImage {
    pub fn new_png(data: Arc<Vec<u8>>) -> Self {
        InMemoryBinImage::Png(ByAddress::from(data))
    }

    pub fn new_jpeg(data: Arc<Vec<u8>>) -> Self {
        InMemoryBinImage::Jpeg(ByAddress::from(data))
    }
}

impl InMemorySvgImage {
    pub fn new(data: Arc<String>) -> Self {
        InMemorySvgImage(ByAddress::from(data))
    }
}
