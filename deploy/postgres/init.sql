CREATE TABLE users (
    id            UUID                  PRIMARY KEY,
    name          VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
)
