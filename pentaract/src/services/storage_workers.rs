use sqlx::PgPool;

use crate::{
    common::jwt_manager::AuthUser,
    errors::{PentaractError, PentaractResult},
    models::storage_workers::{InStorageWorker, StorageWorker},
    repositories::storage_workers::StorageWorkersRepository,
    schemas::storage_workers::InStorageWorkerSchema,
};

pub struct StorageWorkersService<'d> {
    repo: StorageWorkersRepository<'d>,
}

impl<'d> StorageWorkersService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = StorageWorkersRepository::new(db);
        Self { repo }
    }

    pub async fn create(
        &self,
        in_schema: InStorageWorkerSchema,
        user: &AuthUser,
    ) -> PentaractResult<StorageWorker> {
        // checking if user already has a storage worker with such name
        if let Ok(_) = self
            .repo
            .get_by_name_and_user_id(&in_schema.name, user.id)
            .await
        {
            return Err(PentaractError::StorageWorkerNameConflict);
        }

        // creating storage worker
        let in_model = InStorageWorker::new(
            in_schema.name,
            user.id,
            in_schema.token,
            in_schema.storage_id,
        );
        self.repo.create(in_model).await
    }

    pub async fn list(&self, user: &AuthUser) -> PentaractResult<Vec<StorageWorker>> {
        self.repo.list_by_user_id(user.id).await
    }
}
