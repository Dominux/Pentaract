CREATE TABLE users (
    id            UUID         PRIMARY KEY,
    username      VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL
);

CREATE TABLE storage_workers (
    id      UUID         PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    token   VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID         REFERENCES users
);
