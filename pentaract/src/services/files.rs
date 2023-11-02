use sqlx::PgPool;

use crate::{
    common::jwt_manager::AuthUser,
    errors::{PentaractError, PentaractResult},
    models::files::InFile,
    repositories::files::FilesRepository,
    schemas::files::InFileSchema,
};

pub struct FilesService<'d> {
    repo: FilesRepository<'d>,
}

impl<'d> FilesService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = FilesRepository::new(db);
        Self { repo }
    }

    pub async fn upload(&self, in_schema: InFileSchema, user: &AuthUser) -> PentaractResult<()> {
        // 1. saving file in db
        let in_file = InFile::new(in_schema.path, in_schema.storage_id);
        let file = self.repo.create_file(in_file).await?;

        // 2. sending file to storage manager
        todo!()
    }
}
