INSERT INTO messages (sender, data, media_type, timestamp, signature)
VALUES (
    (SELECT participants.id
    FROM participants
    JOIN users ON users.id = participants.identity
    JOIN conversations ON conversations.id = participants.conversation
    WHERE users.email = $1
    AND conversations.id = $2),
    $3, $4, $5, $6
)