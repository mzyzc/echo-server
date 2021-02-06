SELECT conversations.id, conversations.name
FROM conversations
JOIN participants
ON participants.conversation = conversations.id
WHERE participants.identity = (
    SELECT id FROM users WHERE email = $1
)