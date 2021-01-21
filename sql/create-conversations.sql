CREATE TABLE conversations (
    id SERIAL PRIMARY KEY,
    sender VARCHAR(50) references users(email) NOT NULL,
    recipient VARCHAR(50) references users(email) NOT NULL,
    last_timestamp BYTEA
)