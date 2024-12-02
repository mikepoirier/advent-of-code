use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Input Error: {0}")]
    InputError(#[from] input::Error),
    #[error("Character not found")]
    CharNotFound,
    #[error("Parse Int Error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub type Result<T> = core::result::Result<T, Error>;
