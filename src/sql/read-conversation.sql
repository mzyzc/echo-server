SELECT conversations.id, conversations.name
FROM conversations, participants, users
WHERE participant.user = (
    SELECT user.id WHERE user.email = $1
)
