use sqlx::PgPool;
use uuid::Uuid;

use crate::models::users::{InDBUser, User};
use crate::errors::{PentaractError, PentaractResult};

pub struct UsersRepository<'d> {
    db: &'d PgPool
}

impl<'d> UsersRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, in_obj: InDBUser) -> PentaractResult<User> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
                INSERT INTO users (id, email, password)
                VALUES ($1, $2, $3);
            "#
        )
            .bind(id)
            .bind(in_obj.email.clone())
            .bind(in_obj.password_hash.clone())
            .execute(self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_email_key") => {
                    PentaractError::AlreadyExists("user with given email".into())
                }
                _ => PentaractError::Unknown,
            })?;

        let user = User::new(id, in_obj.email, in_obj.password_hash);
        Ok(user)
    }
}
