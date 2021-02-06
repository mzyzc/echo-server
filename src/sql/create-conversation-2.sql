INSERT INTO participants (identity, conversation)
VALUES (
    (SELECT id FROM users WHERE email = $1),
    (SELECT id FROM conversations WHERE name = $2)
)