SELECT users.email, users.display_name, users.public_key
FROM users, participants
WHERE (user.id = participants.user)
AND (participants.conversation = $1)