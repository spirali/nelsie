mod color;
mod document;
mod error;
mod node;
mod page;
mod rectangle;
mod render;
mod resources;
mod shapes;
mod text;
mod types;
mod utils;
pub(crate) mod textutils;
mod image;

pub use color::Color;
pub use document::Document;
pub use error::RendererError as Error;
pub use node::Node;
pub use page::Page;
pub use rectangle::Rectangle;
pub use resources::Resources;
pub use types::{LayoutExpr, LengthOrExpr, NodeId};
pub use image::{ImageId, ImagePlacement};

pub type Result<T> = std::result::Result<T, Error>;
