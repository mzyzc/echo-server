CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    public_key BYTEA NOT NULL,
    display_name VARCHAR(100)
)