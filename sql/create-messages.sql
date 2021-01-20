CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    data BYTEA NOT NULL,
    media_type BYTEA,
    timestamp BYTEA,
    signature BYTEA,
    conversation INT references conversations(id) NOT NULL
)