mod core;
mod globals;
mod layout;
mod pdf;
mod svg;
mod text;
mod image;

pub(crate) use core::{render_slide_step, RenderConfig};
pub(crate) use globals::GlobalResources;
pub(crate) use pdf::PdfBuilder;
