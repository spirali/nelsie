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
pub struct InMemorySvgImage(ByAddress<Arc<xmltree::Element>>);

impl InMemoryBinImage {
    pub fn new_png(data: Arc<Vec<u8>>) -> Self {
        InMemoryBinImage::Png(ByAddress::from(data))
    }

    pub fn new_jpeg(data: Arc<Vec<u8>>) -> Self {
        InMemoryBinImage::Jpeg(ByAddress::from(data))
    }
}

impl InMemorySvgImage {
    pub fn new(data: Arc<xmltree::Element>) -> Self {
        InMemorySvgImage(ByAddress::from(data))
    }

    pub fn as_string(&self) -> String {
        let mut s = Vec::<u8>::new();
        self.0
            .write_with_config(
                &mut s,
                xmltree::EmitterConfig {
                    write_document_declaration: false,
                    ..Default::default()
                },
            )
            .unwrap();
        String::from_utf8(s).unwrap()
    }
}
