use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid Day Error: Please provide a day between 1 and 25.")]
    InvalidDay,
    #[error("No Save Location available")]
    NoSaveLocation,
    #[error("Input processing error")]
    InputProcessing,
    #[error("Parse Int Error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Nom Error: {0}")]
    Nom(String),
}

pub type Result<T> = core::result::Result<T, Error>;
