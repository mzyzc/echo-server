SELECT users.email, participants.display_name, users.public_key
FROM users
JOIN participants ON participants.identity = users.id
JOIN conversations ON conversations.id = participants.conversation
WHERE (conversations.id = $2)
AND ($2 IN (
    SELECT conversation
    FROM participants
    JOIN users ON users.id = participants.identity
    WHERE users.email = $1
))