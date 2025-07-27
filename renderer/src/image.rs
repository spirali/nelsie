use by_address::ByAddress;
use std::sync::Arc;

// #[derive(Debug)]
// pub struct ImagePlacement {
//     pub offset_x: f32,
//     pub offset_y: f32,
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InMemoryImage {
    Png(ByAddress<Arc<Vec<u8>>>),
    Jpeg(ByAddress<Arc<Vec<u8>>>),
    Svg(ByAddress<Arc<String>>),
}

impl InMemoryImage {
    pub fn new_png(data: Arc<Vec<u8>>) -> Self {
        InMemoryImage::Png(ByAddress::from(data))
    }

    pub fn new_jpeg(data: Arc<Vec<u8>>) -> Self {
        InMemoryImage::Jpeg(ByAddress::from(data))
    }

    pub fn new_svg(data: Arc<String>) -> Self {
        InMemoryImage::Svg(ByAddress::from(data))
    }
}
