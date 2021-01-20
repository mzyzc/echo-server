CREATE TABLE conversations (
    id SERIAL PRIMARY KEY,
    sender INT references users(id) NOT NULL,
    recipient INT references users(id) NOT NULL,
    last_timestamp BYTEA
)