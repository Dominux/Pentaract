use std::{env, str::FromStr};

use super::errors::{PentaractError, PentaractResult};

#[derive(Debug, Clone)]
pub struct Config {
    pub db_uri: String,
    pub port: u16,
    pub workers: u16,
    pub channel_capacity: u16,
}

impl Config {
    pub fn new() -> PentaractResult<Self> {
        let db_uri = {
            let db_user: String = Self::get_env_var("DATABASE_USER")?;
            let db_password: String = Self::get_env_var("DATABASE_PASSWORD")?;
            let db_name: String = Self::get_env_var("DATABASE_NAME")?;
            let db_host: String = Self::get_env_var("DATABASE_HOST")?;
            let db_port: String = Self::get_env_var("DATABASE_PORT")?;

            format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}")
        };
        let port = Self::get_env_var("PORT")?;
        let workers = Self::get_env_var("WORKERS")?;
        let channel_capacity = Self::get_env_var("CHANNEL_CAPACITY")?;

        Ok(Self {
            db_uri,
            port,
            workers,
            channel_capacity,
        })
    }

    #[inline]
    fn get_env_var<T: FromStr>(env_var: &str) -> PentaractResult<T> {
        env::var(env_var)
            .map_err(|_| PentaractError::EnvConfigLoadingError(env_var.to_owned()))?
            .parse::<T>()
            .map_err(|_| PentaractError::EnvVarParsingError(env_var.to_owned()))
    }
}
