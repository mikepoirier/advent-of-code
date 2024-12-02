use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No Data Dir")]
    NoDataDir,
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
