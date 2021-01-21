CREATE TABLE users (
    email VARCHAR(50) PRIMARY KEY NOT NULL,
    display_name VARCHAR(32),
    public_key BYTEA NOT NULL,
    pass BYTEA NOT NULL,
    salt BYTEA NOT NULL
)