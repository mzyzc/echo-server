CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL,
    display_name VARCHAR(32),
    public_key BYTEA NOT NULL,
    pass BYTEA NOT NULL,
    salt BYTEA NOT NULL
)