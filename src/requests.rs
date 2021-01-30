use crate::api::{Request, Operation, Target};
use crate::auth::UserAuth;

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::str;
use sqlx::PgPool;

pub async fn handle_request(data: &[u8], user: &mut UserAuth, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;

    // Identify type of request
    match request.operation {
        Operation::Verify => {
            match request.target {
                Target::User => request.verify_users(user, db_pool).await?,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation"))),
            }
        }
        Operation::Create => {
            match request.target {
                Target::Conversation => request.create_conversation(user, db_pool).await?,
                Target::Message => request.create_conversation(user, db_pool).await?,
                Target::User => request.create_user(db_pool).await?,
            }
        }
        Operation::Read => {
            match request.target {
                Target::Conversation => request.read_conversation(user, db_pool).await?,
                Target::Message => request.read_message(user, db_pool).await?,
                Target::User => request.read_user(user, db_pool).await?,
            }
        }
        Operation::Update => {
            match request.target {
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation"))),
            }
        }
        Operation::Delete => {
            match request.target {
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation"))),
            }
        }
    };

    Ok(())
}
