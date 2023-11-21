mod core;
mod globals;
mod image;
mod layout;
mod pdf;
mod svg;
mod text;

pub(crate) use core::{render_slide_step, RenderConfig};
pub(crate) use globals::GlobalResources;
pub(crate) use image::load_image_in_deck;
pub(crate) use pdf::PdfBuilder;
pub(crate) use text::check_fonts;
