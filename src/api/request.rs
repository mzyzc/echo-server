use crate::api;
use crate::auth::{Login, Password};
use crate::api::ApiObject;
use crate::api::response::Response;

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use api::{Conversation, Message, User};
use serde_json::Value;
use sqlx::PgPool;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
    Verify,
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Conversations,
    Messages,
    Users,
}

// Canonical form of a request
#[derive(Debug)]
pub struct Request {
    pub operation: Operation,
    pub target: Target,
    users: Option<Vec<api::User>>,
    messages: Option<Vec<api::Message>>,
    conversations: Option<Vec<api::Conversation>>,
}

impl Request {
    // Separate operation and target from a space-delimited string
    fn split_function(function: &str) -> Result<(String, String), Box<dyn Error>> {
        let split_func: Vec<&str> = function
            .split_ascii_whitespace()
            .collect();

        if split_func.len() < 2 {
            return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Malformed function request")));
        }

        Ok((split_func[0].to_owned(), split_func[1].to_owned()))
    }

    // Create a request object from JSON
    pub fn from_json(data: &str) -> Result<Self, Box<dyn Error>> {
        let data: Value = serde_json::from_str(data)?;

        let (operation, target) = Request::split_function(data["function"].as_str()
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Invalid request function"))?)?;

        let request = Self{
            operation: match operation.to_uppercase().as_ref() {
                "VERIFY" => Operation::Verify,
                "CREATE" => Operation::Create,
                "READ" => Operation::Read,
                "UPDATE" => Operation::Update,
                "DELETE" => Operation::Delete,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown request"))),
            },
            target: match target.to_uppercase().as_ref() {
                "CONVERSATIONS" => Target::Conversations,
                "MESSAGES" => Target::Messages,
                "USERS" => Target::Users,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown target"))),
            },
            users: match data["users"].as_array() {
                Some(d) => {
                    let users = d
                        .iter()
                        .flat_map(|item| api::User::from_json(item))
                        .collect();
                    Some(users)
                },
                None => None,
            },
            messages: match data["messages"].as_array() {
                Some(d) => {
                    let messages = d
                        .iter()
                        .flat_map(|item| api::Message::from_json(item))
                        .collect();
                    Some(messages)
                },
                None => None,
            },
            conversations: match data["conversations"].as_array() {
                Some(d) => {
                    let conversations = d
                        .iter()
                        .flat_map(|item| api::Conversation::from_json(item))
                        .collect();
                    Some(conversations)
                },
                None => None,
            },
        };

        Ok(request)
    }

    // Authenticate a user for the duration of the session
    pub async fn verify_users(self, login: &mut Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Read remote data
        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;
        let user = users[0].clone();

        let email = user.email
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field for 'user'"))?;
        let remote_pass = user.password
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field for 'user'"))?;

        // Read local data
        let stream = sqlx::query_file!("src/sql/verify-user.sql", email)
            .fetch_one(db_pool)
            .await?;

        let local_pass = Password{
            hash: stream.pass,
            salt: stream.salt
        };

        // Validate password
        match local_pass.is_valid(&remote_pass)? {
            true => login.authenticate(email),
            false => return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Invalid password"))),
        };

        Ok(Response{
            status: 1,
            conversations: None,
            messages: None,
            users: None,
        })
    }

    // Add users to the database
    pub async fn create_users(self, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;

        for user in users {
            // Unpack request
            let email = user.email
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field for 'user'"))?;
            let password = user.password
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field for 'user'"))?;
            let public_key = user.public_key
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'public_key' field for 'user'"))?;

            // Salt and hash password
            let password = Password::hash(&password, Option::None)?;

            // Store user data
            sqlx::query_file!("src/sql/create-user.sql",
                    email,
                    public_key,
                    password.hash,
                    password.salt)
                .execute(db_pool)
                .await?;
        };

        Ok(Response{
            status: 1,
            conversations: None,
            messages: None,
            users: None,
        })
    }

    // Add user's conversations to the database
    pub async fn create_conversations(self, login: &Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        if login.is_authenticated == false {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // Unpack request
        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;
        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;

        let conversation = conversations[0].clone();

        let name = conversation.name
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'name' field for 'conversation'"))?;

        // Create conversation
        sqlx::query_file!("src/sql/create-conversation-1.sql", name)
            .execute(db_pool)
            .await?;

        // Add creator user
        sqlx::query_file!("src/sql/create-conversation-2.sql", login.email, name)
            .execute(db_pool)
            .await?;

        // Add remaining users
        for user in users.clone() {
            let email = user.email
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field for 'user'"))?;

            sqlx::query_file!("src/sql/create-conversation-2.sql", email, name)
                .execute(db_pool)
                .await?;
        };

        Ok(Response{
            status: 1,
            conversations: None,
            messages: None,
            users: None,
        })
    }

    // Add messages from a conversation to the database
    pub async fn create_messages(self, login: &Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        if login.is_authenticated == false {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // Unpack request
        let messages = self.messages
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'messages' list"))?;
        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;
        let conversation = &conversations[0];
        let conversation_id = conversation.id
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field for 'conversation'"))?;

        for message in messages {
            let data = message.data
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'data' field for 'message'"))?;
            let media_type = message.media_type
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'media_type' field for 'message'"))?;
            let timestamp = message.timestamp
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'timestamp' field for 'message'"))?;
            let signature = message.signature
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'signature' field for 'message'"))?;

            // Store user data
            sqlx::query_file!("src/sql/create-message.sql",
                    login.email,
                    conversation_id,
                    data,
                    media_type,
                    timestamp,
                    signature)
                .execute(db_pool)
                .await?;
        };
        
        Ok(Response{
            status: 1,
            conversations: None,
            messages: None,
            users: None,
        })
    }

    // Read a user's messages from the database
    pub async fn read_conversations(self, login: &Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        if login.is_authenticated == false {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // Read from database
        let stream = sqlx::query_file!("src/sql/read-conversation.sql", login.email)
            .fetch_all(db_pool)
            .await?;

        // Format response
        let conversations: Vec<Conversation> = stream
            .iter()
            .map(|c| Conversation{
                id: Some(c.id),
                name: Some(c.name.to_owned())
            })
            .collect();

        let response = Response{
            status: 1,
            conversations: Some(conversations),
            messages: None,
            users: None,
        };

        Ok(response)
    }

    // Read messages in a conversation from the database
    pub async fn read_messages(self, login: &Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        if login.is_authenticated == false {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // Unpack request
        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;
        let conversation = &conversations[0];

        let conversation_id = conversation.id
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field for 'conversation'"))?;

        // Read from database
        let stream = sqlx::query_file!("src/sql/read-message.sql",
                login.email,
                conversation_id)
            .fetch_all(db_pool)
            .await?;

        // Format response
        let messages: Vec<Message> = stream
            .iter()
            .map(|m| Message{
                id: None,
                data: Some(m.data.to_owned()),
                media_type: m.media_type.to_owned(),
                timestamp: m.timestamp.to_owned(),
                signature: m.signature.to_owned(),
                sender: Some(m.email.to_owned()),
            })
            .collect();

        let response = Response{
            status: 1,
            conversations: None,
            messages: Some(messages),
            users: None,
        };

        Ok(response)
    }

    // Read users in a conversation from the database
    pub async fn read_users(self, login: &Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
        // Authenticate user
        if login.is_authenticated == false {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // Unpack request
        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;
        let conversation = &conversations[0];

        let conversation_id = conversation.id
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field for 'conversation'"))?;

        // Read from database
        let stream = sqlx::query_file!("src/sql/read-user.sql",
                login.email,
                conversation_id)
            .fetch_all(db_pool)
            .await?;

        // Format response
        let users: Vec<User> = stream
            .iter()
            .map(|u| User{
                id: None,
                email: Some(u.email.to_owned()),
                name: None,
                password: None,
                public_key: Some(u.public_key.to_owned()),
            })
            .collect();

        let response = Response{
            status: 1,
            conversations: None,
            messages: None,
            users: Some(users),
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::request::{Request, Operation, Target};
    use serde_json::json;

    #[test]
    fn test_request_from_json() {
        let json = [
            json!({"function": "CREATE USERS"}).to_string(),
            json!({"function": "READ MESSAGES"}).to_string(),
            json!({"function": "UPDATE CONVERSATIONS"}).to_string(),
            json!({"function": "DELETE MESSAGES"}).to_string(),
            json!({"function": "VERIFY USERS"}).to_string(),
        ];

        let requests: Vec<Request> = json
            .iter()
            .map(|req| Request::from_json(&req).unwrap())
            .collect();

        assert_eq!(requests[0].operation, Operation::Create);
        assert_eq!(requests[0].target, Target::Users);

        assert_eq!(requests[1].operation, Operation::Read);
        assert_eq!(requests[1].target, Target::Messages);

        assert_eq!(requests[2].operation, Operation::Update);
        assert_eq!(requests[2].target, Target::Conversations);

        assert_eq!(requests[3].operation, Operation::Delete);
        assert_eq!(requests[3].target, Target::Messages);

        assert_eq!(requests[4].operation, Operation::Verify);
        assert_eq!(requests[4].target, Target::Users);
    }
}