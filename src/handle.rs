use crate::api::request::{Request, Operation, Target};
use crate::auth::Login;

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::str;
use sqlx::PgPool;

pub async fn handle_request(data: &[u8], user: &mut Login, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;

    // Identify type of request
    match request.operation {
        Operation::Verify => {
            match request.target {
                Target::Users => request.verify_users(user, db_pool).await?,
                _ => return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation"))),
            }
        }
        Operation::Create => {
            match request.target {
                Target::Conversations => request.create_conversations(user, db_pool).await?,
                Target::Messages => request.create_messages(user, db_pool).await?,
                Target::Users => request.create_users(db_pool).await?,
            }
        }
        Operation::Read => {
            match request.target {
                Target::Conversations => request.read_conversations(user, db_pool).await?,
                Target::Messages => request.read_messages(user, db_pool).await?,
                Target::Users => request.read_users(user, db_pool).await?,
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
