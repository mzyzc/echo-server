use crate::api;

use serde_json::{Value, json};

pub struct Response {
    pub status: u8,
    users: Option<Vec<api::User>>,
    messages: Option<Vec<api::Message>>,
    conversations: Option<Vec<api::Conversation>>,
}

impl Response {
    pub fn to_json(&self) -> String {
        let users = &self.users_to_json();
        let messages = &self.messages_to_json();
        let conversations = &self.conversations_to_json();

        json!({
            "status": &self.status,
            "users": users,
            "messages": messages,
            "conversations": conversations,
        }).to_string()
    }

    // Format user array as JSON
    fn users_to_json(&self) -> Option<Value> {
        match &self.users {
            Some(users) => {
                Some(users
                    .iter()
                    .map(|user| json!({
                        "email": user.email,
                        "name": user.name,
                        "publicKey": user.public_key,
                    }))
                    .collect()
                )
            },
            None => None,
        }
    }

    // Format message array as JSON
    fn messages_to_json(&self) -> Option<Value> {
        match &self.messages {
            Some(messages) => {
                Some(messages
                    .iter()
                    .map(|message| json!({
                        "data": message.data,
                        "mediaType": message.media_type,
                        "timestamp": message.timestamp,
                        "signature": message.signature,
                        "sender": message.sender,
                    }))
                    .collect()
                )
            },
            None => None,
        }
    }

    // Format conversation array as JSON
    fn conversations_to_json(&self) -> Option<Value> {
        match &self.conversations {
            Some(conversations) => {
                Some(conversations
                    .iter()
                    .map(|conversation| json!({
                        "id": conversation.id,
                        "name": conversation.name,
                    }))
                    .collect()
                )
            },
            None => None,
        }
    }
}