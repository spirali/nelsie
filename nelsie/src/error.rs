use thiserror::Error;

#[derive(Debug, Error)]
pub enum NelsieError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Renderer(#[from] renderer::Error),
    #[error("{0}")]
    Parsing(String),
    #[error("{0}")]
    Generic(String),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}

impl NelsieError {
    pub fn generic_err(message: impl Into<String>) -> Self {
        NelsieError::Generic(message.into())
    }
    pub fn parsing_err(message: impl Into<String>) -> Self {
        NelsieError::Parsing(message.into())
    }
}

pub type Result<T> = std::result::Result<T, NelsieError>;
