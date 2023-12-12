use uuid::Uuid;

use crate::{
    errors::{PentaractError, PentaractResult},
    models::access::AccessType,
    repositories::access::AccessRepository,
};

pub async fn check_access<'d>(
    repo: &AccessRepository<'d>,
    user_id: Uuid,
    storage_id: Uuid,
    access_type: &AccessType,
) -> PentaractResult<()> {
    if !repo.has_access(user_id, storage_id, access_type).await? {
        Err(PentaractError::DoesNotExist(format!(
            "storage with id \"{storage_id}\""
        )))
    } else {
        Ok(())
    }
}
