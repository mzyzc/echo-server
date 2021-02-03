use crate::auth::{Login, Password};
use crate::data;

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use serde_json::Value;
use sqlx::PgPool;

pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
    Verify,
}

pub enum Target {
    Conversations,
    Messages,
    Users,
}

// Canonical form of a request
pub struct Request {
    pub operation: Operation,
    pub target: Target,
    users: Option<Vec<data::User>>,
    messages: Option<Vec<data::Message>>,
    conversations: Option<Vec<data::Conversation>>,
}

impl Request {
    fn split_function(function: &str) -> (String, String) {
        let split_func: Vec<&str> = function
            .split_ascii_whitespace()
            .collect();

        (split_func[0].to_owned(), split_func[1].to_owned())
    }

    pub fn from_json(data: &str) -> Result<Self, Box<dyn Error>> {
        let data: Value = serde_json::from_str(data)?;

        let (operation, target) = Self::split_function(data["function"].as_str()
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Invalid request function"))?);

        let request = Self{
            operation: match operation.as_ref() {
                "VERIFY" => Operation::Verify,
                "CREATE" => Operation::Create,
                "READ" => Operation::Read,
                "UPDATE" => Operation::Update,
                "DELETE" => Operation::Delete,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown request"))),
            },
            target: match target.as_ref() {
                "CONVERSATIONS" => Target::Conversations,
                "MESSAGES" => Target::Messages,
                "USERS" => Target::Users,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown target"))),
            },
            users: match data["users"].as_array() {
                Some(d) => {
                    let mut users = Vec::new();
                    for item in d.iter() {
                        let user = data::User::from_json(item)?;
                        users.push(user);
                    };
                    Some(users)
                },
                None => None,
            },
            messages: match data["messages"].as_array() {
                Some(d) => {
                    let mut messages = Vec::new();
                    for item in d.iter() {
                        let message = data::Message::from_json(item)?;
                        messages.push(message);
                    };
                    Some(messages)
                },
                None => None,
            },
            conversations: match data["conversations"].as_array() {
                Some(d) => {
                    let mut conversations = Vec::new();
                    for item in d.iter() {
                        let conversation = data::Conversation::from_json(item)?;
                        conversations.push(conversation);
                    };
                    Some(conversations)
                },
                None => None,
            },
        };

        Ok(request)
    }

    pub async fn verify_users(self, login: &mut Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;

        for user in users {
            // Retrieve credentials from database
            let email = user.email
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
            let remote_pass = user.password
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;

            let stream = sqlx::query_file!("src/sql/verify-user.sql", email)
                .fetch_one(db_pool)
                .await?;

            let local_pass = Password{
                hash: stream.pass,
                salt: stream.salt
            };

            // Compare password to user input
            match local_pass.is_valid(&remote_pass)? {
                true => login.authenticate(email),
                false => return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Invalid password"))),
            };
        };

        Ok(())
    }

    pub async fn create_users(self, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;

        for user in users {
            let email = user.email
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
            let password = user.password
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;
            let public_key = user.public_key
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'public_key' field"))?;

            // Salt and hash password
            let password = Password::hash(&password, Option::None)?;

            // Store user data
            /*
            sqlx::query_file!("src/sql/create-user.sql",
                    email,
                    public_key,
                    password.hash,
                    password.salt)
                .execute(db_pool)
                .await?;
            */
        };

        Ok(())
    }

    pub async fn create_conversations(self, login: &Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !login.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'users' list"))?;

        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;

        for conversation in conversations {
            let name = conversation.name
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'name' field"))?;

            let users = users.clone();
            for user in users {
                // Store user data
                /*
                sqlx::query_file!("src/sql/create-conversation.sql", email, name)
                    .execute(db_pool)
                    .await?;
                */
            }
        };

        Ok(())
    }

    pub async fn create_messages(self, login: &Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !login.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        let messages = self.messages
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'messages' list"))?;
        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;

        for (message, conversation) in messages.iter().zip(conversations.iter()) {
            let data = message.data.clone()
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'data' field"))?;
            let media_type = message.media_type.clone()
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'mediaType' field"))?;
            let timestamp = message.timestamp.clone()
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'timestamp' field"))?;
            let signature = message.signature.clone()
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'signature' field"))?;
            let conversation_id = conversation.id
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field"))?;

            // Store user data
            /*
            sqlx::query_file!("src/sql/create-message.sql",
                    data,
                    media_type,
                    timestamp,
                    signature,
                    login.email)
                .execute(db_pool)
                .await?;
            */
        };
        
        Ok(())
    }

    pub async fn read_conversations(self, login: &Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !login.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        /*
        sqlx::query_file!("src/sql/read-conversation.sql", login.email)
            .execute(db_pool)
            .await?;
        */

        Ok(())
    }

    pub async fn read_messages(self, login: &Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !login.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        let conversations = self.conversations
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;

        for conversation in conversations {
            let id = conversation.id
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field"))?;

            /*
            sqlx::query_file!("src/sql/read-message.sql", conversation.id, login.email)
                .execute(db_pool)
                .await?;
            */
        }
        
        Ok(())
    }

    pub async fn read_users(self, login: &Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !login.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        let users = self.users
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'conversations' list"))?;

        for user in users {
            let id = user.id
                .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'id' field"))?;

            /*
            sqlx::query_file!("src/sql/read-user.sql", conversation.id, login.email)
                .execute(db_pool)
                .await?;
            */
        }

        Ok(())
    }
}