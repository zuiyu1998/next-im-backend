use bincode::Error as BincodeError;
use thiserror::Error;
use tonic::Status;
use sea_orm::DbErr;
use redis::RedisError;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("timeout")]
    Timeout,
    #[error("seq not found")]
    SeqNotFound
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] Kind),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("io error: {0}")]
    SerdeError(#[from] BincodeError),
    #[error("db error: {0}")]
    DbErr(#[from] DbErr),
    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        Status::internal(value.to_string())
    }
}
