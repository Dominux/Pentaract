use sqlx::PgPool;

use crate::{errors::PentaractResult, schemas::auth::LoginSchema};

pub struct AuthService<'d> {
    db: &'d PgPool,
}

impl<'d> AuthService<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn login(&self, login_data: LoginSchema) -> PentaractResult<()> {
        todo!()
    }
}
