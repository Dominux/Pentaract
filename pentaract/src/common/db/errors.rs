use crate::errors::PentaractError;

#[inline]
pub fn map_not_found(e: sqlx::Error, entity_name: &str) -> PentaractError {
    match e {
        sqlx::Error::RowNotFound => PentaractError::DoesNotExist(format!("such {entity_name}")),
        _ => {
            tracing::error!("{e}");
            PentaractError::Unknown
        }
    }
}
