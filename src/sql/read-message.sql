SELECT message.data, message.media_type, message.timestamp, message.signature, conversations.sender from messages
FROM messages, conversations
WHERE (conversations.sender = (
    SELECT id FROM users WHERE email = $1
))
OR (conversations.recipient = (
    SELECT id FROM users WHERE email = $1
))