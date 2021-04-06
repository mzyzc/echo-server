pub mod request;
pub mod response;

use std::convert::TryFrom;
use std::error::Error;
use base64;
use serde_json::Value;

trait ApiObject: Sized {
    fn from_json(data: &Value) -> Result<Self, Box<dyn Error>>;
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub public_key: Option<Vec<u8>>,
}

impl ApiObject for User {
    // Create a user object from JSON
    fn from_json(data: &Value) -> Result<User, Box<dyn Error>> {
        Ok(User{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
            email: match data["email"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            name: match data["name"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            password: match data["password"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            public_key: match data["publicKey"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}
#[derive(Debug)]
pub struct Message {
    pub id: Option<i32>,
    pub data: Option<Vec<u8>>,
    pub media_type: Option<Vec<u8>>,
    pub timestamp: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub sender: Option<String>,
}

impl ApiObject for Message {
    // Create a message object from JSON
    fn from_json(data: &Value) -> Result<Message, Box<dyn Error>> {
        Ok(Message{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
            data: match data["data"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            media_type: match data["mediaType"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            timestamp: match data["timestamp"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            signature: match data["signature"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            sender: match data["sender"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Conversation {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl ApiObject for Conversation {
    // Create a conversation object from JSON
    fn from_json(data: &Value) -> Result<Conversation, Box<dyn Error>> {
        Ok(Conversation{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
            name: match data["name"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{User, Message, Conversation};
    use crate::api::ApiObject;
    use serde_json::json;

    #[test]
    fn test_user_from_json() {
        let json = [
            json!({
                "id": 1,
                "email": "1@example.com",
                "name": "Example User",
                "password": "pass",
                "publicKey": "a2V5",
            }),
            json!({}),
        ];

        let users = [
            User::from_json(&json[0]).unwrap(),
            User::from_json(&json[1]).unwrap(),
        ];

        assert_eq!(users[0].id, Some(1));
        assert_eq!(users[0].email, Some(String::from("1@example.com")));
        assert_eq!(users[0].name, Some(String::from("Example User")));
        assert_eq!(users[0].password, Some(String::from("pass")));
        assert_eq!(users[0].public_key, Some(String::from("key").into_bytes()));

        assert_eq!(users[1].id, None);
        assert_eq!(users[1].email, None);
        assert_eq!(users[1].name, None);
        assert_eq!(users[1].password, None);
        assert_eq!(users[1].public_key, None);
    }

    #[test]
    fn test_message_from_json() {
        let json = [
            json!({
                "id": 1,
                "data": "ZGF0YQ==",
                "mediaType": "dGV4dC9wbGFpbg==",
                "timestamp": "dGltZXN0YW1w",
                "signature": "c2lnbmF0dXJl",
                "sender": "1@example.com",
            }),
            json!({}),
        ];

        let messages = [
            Message::from_json(&json[0]).unwrap(),
            Message::from_json(&json[1]).unwrap(),
        ];

        assert_eq!(messages[0].id, Some(1));
        assert_eq!(messages[0].data, Some(String::from("data").into_bytes()));
        assert_eq!(messages[0].media_type, Some(String::from("text/plain").into_bytes()));
        assert_eq!(messages[0].timestamp, Some(String::from("timestamp").into_bytes()));
        assert_eq!(messages[0].signature, Some(String::from("signature").into_bytes()));
        assert_eq!(messages[0].sender, Some(String::from("1@example.com")));

        assert_eq!(messages[1].id, None);
        assert_eq!(messages[1].data, None);
        assert_eq!(messages[1].media_type, None);
        assert_eq!(messages[1].timestamp, None);
        assert_eq!(messages[1].signature, None);
        assert_eq!(messages[1].sender, None);
    }

    #[test]
    fn test_conversation_from_json() {
        let json = [
            json!({
                "id": 1,
                "name": "Example Conversation",
            }),
            json!({}),
        ];

        let conversations = [
            Conversation::from_json(&json[0]).unwrap(),
            Conversation::from_json(&json[1]).unwrap(),
        ];

        assert_eq!(conversations[0].id, Some(1));
        assert_eq!(conversations[0].name, Some(String::from("Example Conversation")));

        assert_eq!(conversations[1].id, None);
        assert_eq!(conversations[1].name, None);
    }
}