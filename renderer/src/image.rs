use by_address::ByAddress;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct ImageId(u32);

impl ImageId {
    pub fn new(image_id: u32) -> Self {
        ImageId(image_id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn bump(&mut self) -> ImageId {
        self.0 += 1;
        ImageId::new(self.0)
    }
}

#[derive(Debug)]
pub struct ImagePlacement {
    pub image_id: ImageId,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InMemoryImage {
    Png(ByAddress<Arc<[u8]>>),
    Jpeg(ByAddress<Arc<[u8]>>),
    Svg(String),
}
