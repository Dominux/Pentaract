use pwhash::bcrypt;

use crate::errors::{PentaractError, PentaractResult};

pub struct PasswordManager;

impl PasswordManager {
    pub fn generate(password: &str) -> PentaractResult<String> {
        bcrypt::hash(password).map_err(|e| {
            tracing::error!("{e}");
            PentaractError::Unknown
        })
    }

    pub fn verify(password: &str, hash: &str) -> PentaractResult<()> {
        if bcrypt::verify(password, hash) {
            Ok(())
        } else {
            Err(PentaractError::NotAuthenticated)
        }
    }
}
