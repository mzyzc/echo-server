CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    media_type VARCHAR(50) NOT NULL,
    time_sent TIMESTAMPTZ,
    signature BYTEA,
    conversation INT references conversations(id) NOT NULL
)