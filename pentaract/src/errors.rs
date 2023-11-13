use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PentaractError {
    #[error("environment variable `{0}` is not set")]
    EnvConfigLoadingError(String),
    #[error("environment variable `{0}` cannot be parsed")]
    EnvVarParsingError(String),

    #[error("user was removed")]
    UserWasRemoved,

    #[error("{0} already exists")]
    AlreadyExists(String),
    #[error("`{0}` does not exist")]
    DoesNotExist(String),
    #[error("User already has a storage with such name")]
    StorageNameConflict,
    #[error("User already has a storage worker with such name")]
    StorageWorkerNameConflict,
    #[error("Token must be unique")]
    StorageWorkerTokenConflict,
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("Telegram API error: `{0}`")]
    TelegramAPIError(String),
    #[error("You need to add at least 1 storage worker")]
    NoStorageWorkers,
    #[error("Invalid path")]
    InvalidPath,
    #[error("unknown error")]
    Unknown,
}

impl From<PentaractError> for (StatusCode, String) {
    fn from(e: PentaractError) -> Self {
        match &e {
            PentaractError::AlreadyExists(_) => (StatusCode::CONFLICT, e.to_string()),
            PentaractError::NotAuthenticated => (StatusCode::UNAUTHORIZED, e.to_string()),
            PentaractError::DoesNotExist(_) => (StatusCode::NOT_FOUND, e.to_string()),
            _ => {
                tracing::error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_owned(),
                )
            }
        }
    }
}

impl From<reqwest::Error> for PentaractError {
    fn from(e: reqwest::Error) -> Self {
        match e.status() {
            Some(e) if e.is_client_error() => PentaractError::TelegramAPIError(e.to_string()),
            Some(_) | None => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        }
    }
}

pub type PentaractResult<T> = Result<T, PentaractError>;
