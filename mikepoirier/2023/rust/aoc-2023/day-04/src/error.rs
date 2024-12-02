use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Input Error: {0}")]
    Input(#[from] input::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
