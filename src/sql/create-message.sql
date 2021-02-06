INSERT INTO messages (sender, data, media_type, timestamp, signature)
VALUES (
    (SELECT id FROM users WHERE email = $1),
    $2, $3, $4, $5
)