use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PentaractError {
    #[error("environment variable `{0}` is not set")]
    EnvConfigLoadingError(String),
    #[error("environment variable `{0}` cannot be parsed")]
    EnvVarParsingError(String),

    #[error("`{0}` already exists")]
    AlreadyExists(String),
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("unknown error")]
    Unknown,
}

impl From<PentaractError> for (StatusCode, String) {
    fn from(e: PentaractError) -> Self {
        match &e {
            PentaractError::AlreadyExists(_) => (StatusCode::CONFLICT, e.to_string()),
            PentaractError::NotAuthenticated => (StatusCode::UNAUTHORIZED, e.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        }
    }
}

pub type PentaractResult<T> = Result<T, PentaractError>;
