use sqlx::PgPool;
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::users::{InDBUser, User};

pub struct UsersRepository<'d> {
    db: &'d PgPool,
}

impl<'d> UsersRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, in_obj: InDBUser) -> PentaractResult<User> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
                INSERT INTO users (id, username, password_hash)
                VALUES ($1, $2, $3);
            "#,
        )
        .bind(id)
        .bind(in_obj.username.clone())
        .bind(in_obj.password_hash.clone())
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                PentaractError::AlreadyExists("user with given username".into())
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let user = User::new(id, in_obj.username, in_obj.password_hash);
        Ok(user)
    }

    pub async fn get_by_username(&self, username: &str) -> PentaractResult<User> {
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(self.db)
            .await
            .map_err(|e| map_not_found(e, "user"))
    }
}
