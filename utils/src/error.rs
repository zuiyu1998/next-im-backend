use abi::reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] Kind),

    #[error("internal server errors")]
    InternalServer(String),

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),

    #[error("tonic error: {0}")]
    TonicError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
