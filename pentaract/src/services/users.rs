use sqlx::PgPool;

use crate::{
    common::password_manager::PasswordManager, errors::PentaractResult, models::users::InDBUser,
    repositories::users::UsersRepository, schemas::users::InUser,
};

pub struct UsersService<'d> {
    repo: UsersRepository<'d>,
}

impl<'d> UsersService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = UsersRepository::new(db);
        Self { repo }
    }

    pub async fn create(&self, in_user: InUser) -> PentaractResult<()> {
        let password_hash = PasswordManager::generate(&in_user.password).unwrap();
        let user = InDBUser::new(in_user.email, password_hash);
        self.repo.create(user).await?;
        Ok(())
    }
}
