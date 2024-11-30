use abi::{bincode::Error as BincodeError, Error as AbiError};
use thiserror::Error;
use utils::Error as UtilsError;

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
    #[error("io error: {0}")]
    AbiError(#[from] AbiError),
    #[error("io error: {0}")]
    UtilsError(#[from] UtilsError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
