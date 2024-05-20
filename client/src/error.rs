use abi::Error as AbiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("ServerNotResponding")]
    ServerNotResponding,
    #[error("ServerError")]
    ServerError,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("connect error: {0}")]
    Kind(#[from] Kind),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
