use allsorts::error::{ParseError, ReadWriteError, ShapingError};
use allsorts::subset::SubsetError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Font parse error: {0}")]
    FontParse(#[from] ParseError),
    #[error("Font read or write error: {0}")]
    FontReadWrite(#[from] ReadWriteError),
    #[error("Font typeset error: {0}")]
    FontShaping(#[from] ShapingError),
    #[error("Font parse error: {0}")]
    FontSubset(#[from] SubsetError),
    #[error("Font subsetting error: {0}")]
    LockError(String),
    #[error("Font '{0}' cannot be read, it seems to be empty")]
    MalformedFont(String),
    #[error("Error creating pdf document: {0}")]
    PdfWrite(String),
    #[error("Font '{0}' is unknown")]
    UnknownFont(String),
}
