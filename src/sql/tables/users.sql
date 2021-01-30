CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL,
    public_key BYTEA NOT NULL,
    pass BYTEA NOT NULL,
    salt BYTEA NOT NULL
)