CREATE TABLE users (
    id            UUID         PRIMARY KEY,
    username      VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL
);

CREATE TABLE storages (
    id      UUID         PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    chat_id BigInt       NOT NULL UNIQUE,
    user_id UUID         NOT NULL REFERENCES users
                                  ON DELETE CASCADE 
                                  ON UPDATE CASCADE
);

CREATE TABLE storage_workers (
    id         UUID         PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    token      VARCHAR(255) NOT NULL UNIQUE,
    user_id    UUID         NOT NULL REFERENCES users
                                     ON DELETE CASCADE 
                                     ON UPDATE CASCADE,
    storage_id UUID         REFERENCES storages
);

CREATE TABLE files (
    id          UUID         PRIMARY KEY,
    path        VARCHAR      NOT NULL,
    size        BigInt       NOT NULL,
    storage_id  UUID         NOT NULL REFERENCES storages
                                      ON DELETE CASCADE 
                                      ON UPDATE CASCADE,
    is_uploaded bool         NOT NULL,

    UNIQUE (path, storage_id)
);

CREATE TABLE file_chunks (
    id               UUID         PRIMARY KEY,
    file_id          UUID         NOT NULL REFERENCES files 
                                           ON DELETE CASCADE 
                                           ON UPDATE CASCADE,
    telegram_file_id VARCHAR(255) NOT NULL,
    position         SmallInt     NOT NULL
);

CREATE TABLE storage_workers_usages (
    id                 UUID      PRIMARY KEY,
    storage_worker_id  UUID      NOT NULL REFERENCES storage_workers
                                          ON DELETE CASCADE 
                                          ON UPDATE CASCADE,
    dt                 TIMESTAMP DEFAULT NOW()
);
