INSERT INTO messages (data, media_type, timestamp, signature, sender)
VALUES ($1, $2, $3, $4, (
        SELECT id
        FROM users
        WHERE email = $5
    )
)