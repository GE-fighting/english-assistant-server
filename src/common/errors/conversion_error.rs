use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to parse datetime: {0}")]
    DateTimeParse(#[from] time::error::Parse),
    #[error("Failed to format datetime: {0}")]
    DateTimeFormat(#[from] time::error::Format),
}
