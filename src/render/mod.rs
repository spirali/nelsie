mod core;
mod globals;
mod layout;
mod svg;
mod text;
mod pdf;

pub(crate) use core::{render_slide_step, RenderConfig};
pub(crate) use globals::GlobalResources;
pub(crate) use pdf::PdfBuilder;
