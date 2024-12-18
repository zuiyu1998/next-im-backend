use abi::Error as AbiError;
use thiserror::Error;
use abi::reqwest::Error as ReqwestError;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("ServerNotResponding")]
    ServerNotResponding,
    #[error("ServerError")]
    ServerError,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] Kind),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
