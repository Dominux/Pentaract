use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::{access::check_access, jwt_manager::AuthUser},
    errors::{PentaractError, PentaractResult},
    models::{
        access::{AccessType, UserWithAccess},
        storages::{InStorage, Storage, StorageWithInfo},
    },
    repositories::{access::AccessRepository, storages::StoragesRepository},
    schemas::{
        access::{GrantAccess, RestrictAccess},
        storages::InStorageSchema,
    },
};

pub struct StoragesService<'d> {
    repo: StoragesRepository<'d>,
    access_repo: AccessRepository<'d>,
}

impl<'d> StoragesService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = StoragesRepository::new(db);
        let access_repo = AccessRepository::new(db);
        Self { repo, access_repo }
    }

    pub async fn create(
        &self,
        in_schema: InStorageSchema,
        user: &AuthUser,
    ) -> PentaractResult<Storage> {
        // checking if user already has a storage with such name
        if let Ok(_) = self
            .repo
            .get_by_name_and_user_id(&in_schema.name, user.id)
            .await
        {
            return Err(PentaractError::StorageNameConflict);
        }

        // creating storage
        let in_model = InStorage::new(in_schema.name, in_schema.chat_id);
        let storage = self.repo.create(in_model).await?;

        // setting user as the storage admin
        let access_schema = GrantAccess::new(user.email.clone(), AccessType::A);
        let result = self
            .access_repo
            .create_or_update(storage.id, access_schema)
            .await;
        if result.is_err() {
            // fallback
            self.repo.delete_storage(storage.id).await?
        }
        result.map(|_| storage)
    }

    pub async fn list(&self, user: &AuthUser) -> PentaractResult<Vec<StorageWithInfo>> {
        self.repo.list_by_user_id(user.id).await
    }

    pub async fn get(&self, id: Uuid, user: &AuthUser) -> PentaractResult<Storage> {
        check_access(&self.access_repo, user.id, id, &AccessType::R).await?;

        self.repo.get_by_id(id).await
    }

    pub async fn delete(&self, id: Uuid, user: &AuthUser) -> PentaractResult<()> {
        check_access(&self.access_repo, user.id, id, &AccessType::A).await?;

        self.repo.delete_storage(id).await
    }

    pub async fn grant_access(
        &self,
        id: Uuid,
        in_schema: GrantAccess,
        user: &AuthUser,
    ) -> PentaractResult<()> {
        check_access(&self.access_repo, user.id, id, &AccessType::A).await?;

        if in_schema.user_email == user.email {
            return Err(PentaractError::CannotManageAccessOfYourself);
        }

        self.access_repo.create_or_update(id, in_schema).await
    }

    pub async fn list_users_with_access(
        &self,
        id: Uuid,
        user: &AuthUser,
    ) -> PentaractResult<Vec<UserWithAccess>> {
        check_access(&self.access_repo, user.id, id, &AccessType::A).await?;

        self.access_repo.list_users_with_access(id).await
    }

    pub async fn restrict_access(
        &self,
        id: Uuid,
        in_schema: RestrictAccess,
        user: &AuthUser,
    ) -> PentaractResult<()> {
        check_access(&self.access_repo, user.id, id, &AccessType::A).await?;

        if in_schema.user_id == user.id {
            return Err(PentaractError::CannotManageAccessOfYourself);
        }

        self.access_repo.delete_access(in_schema.user_id, id).await
    }
}
