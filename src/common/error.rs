use thiserror::Error;

#[derive(Debug, Error)]
pub enum NelsieError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    XmlError(#[from] roxmltree::Error),
    #[error(transparent)]
    SvgError(#[from] usvg::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("{0}")]
    ParsingError(String),
    #[error("{0}")]
    GenericError(String),
}

impl NelsieError {
    pub fn generic_err(message: impl Into<String>) -> Self {
        NelsieError::GenericError(message.into())
    }
    pub fn parsing_err(message: impl Into<String>) -> Self {
        NelsieError::ParsingError(message.into())
    }
}

pub type Result<T> = std::result::Result<T, NelsieError>;
