use crate::auth::{UserAuth, Password};

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use base64;
use serde::Deserialize;
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
    data: Option<Vec<u8>>,
    display_name: Option<String>,
    email: Option<String>,
    media_type: Option<Vec<u8>>,
    password: Option<String>,
    public_key: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
    timestamp: Option<Vec<u8>>,
}

impl Request {
    pub fn from_json(data: &str) -> Result<Request, Box<dyn Error>> {
        let request: RawRequest = serde_json::from_str(data)?;
        return request.decode()
    }

    pub async fn verify_users(self, user: &mut UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let email = self.email
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
        let remote_pass = self.password
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;

        // Retrieve credentials from database
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

        Ok(())
    }

    pub async fn create_user(self, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let email = self.email
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
        let password = self.password
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;
        let public_key = self.public_key
            .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'public_key' field"))?;

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

// Intermediate form of a request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRequest {
    function: String,
    data: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    media_type: Option<String>,
    password: Option<String>,
    public_key: Option<String>,
    signature: Option<String>,
    timestamp: Option<String>,
}

impl RawRequest {
    // Convert data to canonical form
    fn decode(&self) -> Result<Request, Box<dyn Error>> {
        let split_func: Vec<&str> = self.function
            .split_ascii_whitespace()
            .collect();

        return Ok(Request{
            operation: match split_func[0] {
                "VERIFY" => Operation::Verify,
                "CREATE" => Operation::Create,
                "READ" => Operation::Read,
                "UPDATE" => Operation::Update,
                "DELETE" => Operation::Delete,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown request"))),
            },
            target: match split_func[1] {
                "CONVERSATION" => Target::Conversation,
                "MESSAGE" => Target::Message,
                "USER" => Target::User,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Unknown target"))),
            },
            data: match &self.data {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            display_name: match &self.display_name {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            email: match &self.email {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            media_type: match &self.media_type {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            password: match &self.password {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            public_key: match &self.public_key {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            signature: match &self.signature {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            timestamp: match &self.timestamp {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}