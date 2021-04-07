CREATE TABLE conversations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    timestamp BYTEA
)