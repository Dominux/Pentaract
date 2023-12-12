use std::time::Duration;

use sqlx::PgPool;

use crate::{
    common::{
        jwt_manager::{AuthUser, JWTManager},
        password_manager::PasswordManager,
    },
    config::Config,
    errors::{PentaractError, PentaractResult},
    repositories::users::UsersRepository,
    schemas::auth::LoginSchema,
};

pub struct AuthService<'d> {
    repo: UsersRepository<'d>,
}

impl<'d> AuthService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        let repo = UsersRepository::new(db);
        Self { repo }
    }

    pub async fn login(
        &self,
        login_data: LoginSchema,
        config: &Config,
    ) -> PentaractResult<(String, Duration)> {
        // trying to find a user with a given email
        let user = self
            .repo
            .get_by_email(&login_data.email)
            .await
            .map_err(|_| PentaractError::NotAuthenticated)?;

        // verifying password
        PasswordManager::verify(&login_data.password, &user.password_hash)?;

        // generating access token
        let user = AuthUser::new(user.id, login_data.email);
        let expire_in = Duration::from_secs(config.access_token_expire_in_secs.into());
        let token = JWTManager::generate(user, expire_in, &config.secret_key);
        Ok((token, expire_in))

        // TODO: add generating refresh token
    }
}
