use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("kind: {0}")]
    Kind(#[from] Kind),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
