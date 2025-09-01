use svg2pdf::usvg;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // #[error(transparent)]
    // Xml(#[from] usvg::roxmltree::Error),
    #[error(transparent)]
    Svg(#[from] usvg::Error),
    // #[error(transparent)]
    // Zip(#[from] zip::result::ZipError),
    #[error("{0}")]
    Parsing(String),
    #[error("{0}")]
    Generic(String),
}

impl RendererError {
    pub fn generic_err(message: impl Into<String>) -> Self {
        RendererError::Generic(message.into())
    }
    pub fn parsing_err(message: impl Into<String>) -> Self {
        RendererError::Parsing(message.into())
    }
}
