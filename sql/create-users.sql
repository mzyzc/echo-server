CREATE TABLE users (
    email VARCHAR(50) NOT NULL,
    display_name VARCHAR(32)
    public_key BYTEA NOT NULL,
    password BYTEA NOT NULL,
    salt BYTEA NOT NULL,
)