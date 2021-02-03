use crate::auth::{UserAuth, Password};
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
    Conversation,
    Message,
    User,
}

// Canonical form of a request
pub struct Request {
    pub operation: Operation,
    pub target: Target,
    user: Option<Vec<data::User>>,
    message: Option<Vec<data::Message>>,
    conversation: Option<Vec<data::Conversation>>,
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
                "CONVERSATION" => Target::Conversation,
                "MESSAGE" => Target::Message,
                "USER" => Target::User,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown target"))),
            },
            user: match data["user"].as_array() {
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
            message: match data["message"].as_array() {
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
            conversation: match data["conversation"].as_array() {
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

    pub async fn verify_users(self, user: &mut UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        // TODO

        // Retrieve credentials from database
        /*
        let stream = sqlx::query_file!("src/sql/verify-user.sql", email)
            .fetch_one(db_pool)
            .await?;

        let local_pass = Password{
            hash: stream.pass,
            salt: stream.salt
        };

        // Compare password to user input
        match local_pass.is_valid(&remote_pass)? {
            true => user.authenticate(email),
            false => return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Invalid password"))),
        };
        */

        Ok(())
    }

    pub async fn create_user(self, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        // TODO

        // Salt and hash password
        /*
        let password = Password::hash(&password, Option::None)?;

        // Store user data
        sqlx::query_file!("src/sql/create-user.sql",
                email,
                public_key,
                password.hash,
                password.salt)
            .execute(db_pool)
            .await?;
        */

        Ok(())
    }

    pub async fn create_conversation(self, user: &UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !user.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // TODO

        Ok(())
    }

    pub async fn create_message(self, user: &UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !user.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // TODO
        
        Ok(())
    }

    pub async fn read_conversation(self, user: &UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !user.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // TODO

        Ok(())
    }

    pub async fn read_message(self, user: &UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !user.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // TODO
        
        Ok(())
    }

    pub async fn read_user(self, user: &UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        if !user.is_authenticated {
            return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Not authenticated")));
        }

        // TODO

        Ok(())
    }
}