use thiserror::Error;

#[derive(Debug, Error)]
pub enum PentaractError {
    #[error("environment variable `{0}` is not set")]
    EnvConfigLoadingError(String),
    #[error("environment variable `{0}` cannot be parsed")]
    EnvVarParsingError(String),

    #[error("`{0}` already exists")]
    AlreadyExists(String),
    #[error("unknown error")]
    Unknown,
}

pub type PentaractResult<T> = Result<T, PentaractError>;
