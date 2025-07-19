mod color;
mod document;
mod error;
mod image;
mod node;
mod page;
mod rectangle;
mod render;
mod resources;
mod shapes;
mod text;
pub(crate) mod textutils;
mod types;
mod utils;

pub use color::Color;
pub use document::Document;
pub use error::RendererError as Error;
pub use image::{ImageId, ImagePlacement};
pub use node::Node;
pub use page::Page;
pub use rectangle::Rectangle;
pub use resources::Resources;
pub use types::{LayoutExpr, LengthOrExpr, NodeId};

pub type Result<T> = std::result::Result<T, Error>;
