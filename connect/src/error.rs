use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("timeout")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("connect error: {0}")]
    Kind(#[from] Kind),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
