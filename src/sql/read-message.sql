SELECT messages.data, messages.media_type, messages.timestamp, messages.signature, users.email
FROM messages
JOIN participants ON participants.id = messages.sender
JOIN users ON users.id = participants.identity
JOIN conversations ON conversations.id = participants.conversation
WHERE (users.email = $1)
AND (conversations.id = $2)