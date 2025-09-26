use thiserror::Error;

#[derive(Error, Debug)]
pub enum NameError {
    #[error("Empty string provided for name")]
    EmptyString,
}

#[derive(Error, Debug)]
pub enum CitationError {
    #[error("Invalid citation format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
}