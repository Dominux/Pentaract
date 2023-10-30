use sqlx::PgPool;

use crate::{
    common::jwt_manager::AuthUser,
    errors::{PentaractError, PentaractResult},
    models::storages::{InStorage, Storage},
    repositories::storages::StoragesRepository,
    schemas::storages::InStorageSchema,
};

pub struct StoragesService<'d> {
    repo: StoragesRepository<'d>,
}

impl<'d> StoragesService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = StoragesRepository::new(db);
        Self { repo }
    }

    pub async fn create(
        &self,
        in_schema: InStorageSchema,
        user: &AuthUser,
    ) -> PentaractResult<Storage> {
        // checking if user already has a storage worker with such name
        if let Ok(_) = self
            .repo
            .get_by_name_and_user_id(&in_schema.name, user.id)
            .await
        {
            return Err(PentaractError::StorageNameConflict);
        }

        // creating storage worker
        let in_model = InStorage::new(in_schema.name, user.id, in_schema.chat_id);
        self.repo.create(in_model).await
    }

    pub async fn list(&self, user: &AuthUser) -> PentaractResult<Vec<Storage>> {
        self.repo.list_by_user_id(user.id).await
    }
}
