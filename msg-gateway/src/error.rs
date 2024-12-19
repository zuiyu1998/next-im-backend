use abi::{bincode::Error as BincodeError, Error as AbiError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("MsgInvaild")]
    MsgInvaild,
    #[error("UseNotLogin")]
    UseNotLogin,
    #[error("UseTokenInvaild")]
    UseTokenInvaild
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] ErrorKind),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("io error: {0}")]
    SerdeError(#[from] BincodeError),
    #[error("io error: {0}")]
    AbiError(#[from] AbiError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
