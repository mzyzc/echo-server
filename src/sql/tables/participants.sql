CREATE TABLE participants (
    id SERIAL PRIMARY KEY,
    display_name VARCHAR(32)
    user INT references users(id) NOT NULL,
    conversation INT references conversations(id) NOT NULL,
)