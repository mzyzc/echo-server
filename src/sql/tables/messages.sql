CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    data BYTEA NOT NULL,
    media_type BYTEA,
    timestamp BYTEA,
    signature BYTEA,
    sender INT references participants(id) NOT NULL
)