use bincode::Error as BincodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("timeout")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] Kind),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("io error: {0}")]
    SerdeError(#[from] BincodeError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
