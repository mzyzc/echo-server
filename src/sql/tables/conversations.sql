CREATE TABLE conversations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    timestamp BYTEA
)