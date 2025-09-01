mod color;
mod document;
mod error;
mod image;
mod layout_info;
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
pub use document::{Document, Register, RenderingOptions};
pub use error::RendererError as Error;
pub use image::{InMemoryBinImage, InMemorySvgImage};
pub use layout_info::PageLayout;
pub use node::{ContentId, Node, NodeChild};
pub use page::Page;
pub use rectangle::Rectangle;
pub use resources::Resources;
pub use shapes::{Arrow, FillAndStroke, Path, PathPart, Shape, ShapeRect, Stroke};
pub use taffy;
pub use text::{
    FontStretch, InlineId, ParsingChars, SyntaxHighlightSettings, Text, TextAlign, TextStyle,
    TextStyling,
};
pub use types::{LayoutExpr, Length, LengthOrAuto, LengthOrExpr, NodeId};
pub type Result<T> = std::result::Result<T, Error>;
