use std::time::Duration;

use sqlx::PgPool;

use crate::{
    common::{db::pool::get_pool, password_manager::PasswordManager},
    config::Config,
    errors::PentaractError,
    models::users::InDBUser,
    repositories::users::UsersRepository,
};

#[inline]
pub async fn create_db(dsn: &str, dbname: &str, max_connection: u32, timeout: Duration) {
    let db = get_pool(dsn, max_connection, timeout).await;

    tracing::debug!("creating database");

    let result = sqlx::query(format!("CREATE DATABASE {dbname}").as_str())
        .execute(&db)
        .await;

    match &result {
        Ok(_) => {
            tracing::debug!("created database");
            return;
        }
        Err(sqlx::Error::Database(dbe)) => {
            if let Some(code) = dbe.code() {
                if code == "42P04" {
                    tracing::debug!("database already exists; skipping");
                    return;
                }
            }
        }
        _ => (),
    };

    result.unwrap();
}

#[inline]
pub async fn init_db(db: &PgPool) {
    tracing::debug!("initing database");

    let mut transaction = db.begin().await.unwrap();

    for statement in [
        "
        CREATE TABLE IF NOT EXISTS users (
            id            UUID         PRIMARY KEY,
            email         VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL
        );
    ",
        "
        CREATE TABLE IF NOT EXISTS storages (
            id      UUID         PRIMARY KEY,
            name    VARCHAR(255) NOT NULL,
            chat_id BigInt       NOT NULL UNIQUE
        );

    ",
        "
        CREATE TABLE IF NOT EXISTS storage_workers (
            id         UUID         PRIMARY KEY,
            name       VARCHAR(255) NOT NULL,
            token      VARCHAR(255) NOT NULL UNIQUE,
            user_id    UUID         NOT NULL REFERENCES users
                                            ON DELETE CASCADE 
                                            ON UPDATE CASCADE,
            storage_id UUID         REFERENCES storages
        );

    ",
        "
        DO
        $$
        BEGIN
        IF NOT EXISTS (
            SELECT *
            FROM pg_type typ
            INNER JOIN pg_namespace nsp ON nsp.oid = typ.typnamespace
            WHERE nsp.nspname = current_schema() AND typ.typname = 'access_type'
        ) THEN
            CREATE TYPE access_type AS ENUM ('r', 'w', 'a');
        END IF;
        END;
        $$;
    ",
        "
        CREATE TABLE IF NOT EXISTS access (
            id          UUID        PRIMARY KEY,
            user_id     UUID        NOT NULL REFERENCES users
                                            ON DELETE CASCADE 
                                            ON UPDATE CASCADE,
            storage_id  UUID        NOT NULL REFERENCES storages
                                            ON DELETE CASCADE 
                                            ON UPDATE CASCADE,
            access_type access_type NOT NULL,

            UNIQUE(user_id, storage_id)
        );
    ",
        "
        CREATE TABLE IF NOT EXISTS files (
            id          UUID         PRIMARY KEY,
            path        VARCHAR      NOT NULL,
            size        BigInt       NOT NULL,
            storage_id  UUID         NOT NULL REFERENCES storages
                                            ON DELETE CASCADE 
                                            ON UPDATE CASCADE,
            is_uploaded bool         NOT NULL,

            UNIQUE (path, storage_id)
        );
    ",
        "
        CREATE TABLE IF NOT EXISTS file_chunks (
            id               UUID         PRIMARY KEY,
            file_id          UUID         NOT NULL REFERENCES files 
                                                ON DELETE CASCADE 
                                                ON UPDATE CASCADE,
            telegram_file_id VARCHAR(255) NOT NULL,
            position         SmallInt     NOT NULL
        );
    ",
        "
        CREATE TABLE IF NOT EXISTS storage_workers_usages (
            id                 UUID      PRIMARY KEY,
            storage_worker_id  UUID      NOT NULL REFERENCES storage_workers
                                                ON DELETE CASCADE 
                                                ON UPDATE CASCADE,
            dt                 TIMESTAMP DEFAULT NOW()
        );
    ",
        r#"
        CREATE OR REPLACE FUNCTION public.regexp_quote(IN TEXT)
            RETURNS TEXT
            LANGUAGE plpgsql
            STABLE
        AS $$
            /*******************************************************************************
            * Function Name: regexp_quote
            * In-coming Param:
            *   The string to decoded and convert into a set of text arrays.
            * Returns:
            *   This function produces a TEXT that can be used as a regular expression
            *   pattern that would match the input as if it were a literal pattern.
            * Description:
            *   Takes in a TEXT in and escapes all of the necessary characters so that
            *   the output can be used as a regular expression to match the input as if
            *   it were a literal pattern.
            * Source: https://cwestblog.com/2012/07/10/postgresql-escape-regular-expressions/ * 
            *     The original one doesn't work anymore.
            ******************************************************************************/
        BEGIN
            RETURN REGEXP_REPLACE($1, '([\.\+\*\?\^\$\(\)\[\]\{\}\|\\])', '\\\1', 'g');
        END;
        $$;
    "#,
    ] {
        sqlx::query(statement)
            .execute(&mut *transaction)
            .await
            .map_err(|e| {
                tracing::error!("error during initing database with query:\n{statement}");
                e
            })
            .unwrap();
    }

    transaction.commit().await.unwrap();
}

#[inline]
pub async fn create_superuser(db: &PgPool, config: &Config) {
    let password_hash = PasswordManager::generate(&config.superuser_pass).unwrap();
    let user = InDBUser::new(config.superuser_email.clone(), password_hash);
    let result = UsersRepository::new(&db).create(user).await;

    match result {
        Ok(_) => tracing::debug!("created superuser"),

        // ignoring conflict error -> just skipping it
        Err(e) if matches!(e, PentaractError::AlreadyExists(_)) => {
            tracing::debug!("superuser already exists; skipping")
        }

        // in case of another error kind -> terminating process
        _ => {
            panic!("can't create superuser; terminating process")
        }
    };
}
