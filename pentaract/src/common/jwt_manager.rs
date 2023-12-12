use std::{
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{PentaractError, PentaractResult};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub(self) sub: String,
    pub(self) email: String,
    pub(self) exp: usize,
}

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

impl AuthUser {
    pub fn new(id: Uuid, email: String) -> Self {
        Self { id, email }
    }
}

pub struct JWTManager;

impl JWTManager {
    pub fn generate(user: AuthUser, expire_in: Duration, secret_key: &str) -> String {
        let expire_date = SystemTime::now() + expire_in;
        let expire_timestamp = expire_date.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let claims = Claims {
            sub: user.id.into(),
            email: user.email,
            exp: expire_timestamp,
        };
        let key = EncodingKey::from_secret(secret_key.as_bytes());

        encode(&Header::default(), &claims, &key).unwrap()
    }

    pub fn validate(token: &str, secret_key: &str) -> PentaractResult<AuthUser> {
        let validation = Validation::new(Algorithm::HS256);
        let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|token_data| {
                let id = token_data.claims.sub;
                let id = Uuid::from_str(&id).unwrap(); // token is valid so uuid is too
                AuthUser::new(id, token_data.claims.email)
            })
            .map_err(|_| PentaractError::NotAuthenticated)
    }
}
