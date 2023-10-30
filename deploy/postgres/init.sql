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
);

CREATE TABLE storage_workers (
    id         UUID         PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    token      VARCHAR(255) NOT NULL UNIQUE,
    user_id    UUID         NOT NULL REFERENCES users,
    storage_id UUID         REFERENCES storages
);
