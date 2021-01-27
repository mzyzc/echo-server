use crate::auth::Password;
use crate::request::{Request, Operation, Target};

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::str;
use sqlx::PgPool;

pub async fn handle_request(data: &[u8], db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;
    let mut user: Option<String> = Option::None;

    // Identify type of request
    match request.operation {
        Operation::Verify => {
            match request.target {
                Target::User => {  // VERIFY USER
                    let email = request.email
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
                    let password = request.password
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;

                    // Retrieve credentials from database
                    let stream = sqlx::query_file!("src/sql/verify-user.sql", email)
                        .fetch_one(db_pool)
                        .await?;

                    let thing = Password{hash: stream.pass, salt: stream.salt};

                    // Compare password to user input
                    match thing.is_valid(&password)? {
                        true => { user = Some(email) },
                        false => return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Invalid password"))),
                    };
                }
                Target::Message => {  // VERIFY MESSAGE
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Create => {
            match request.target {
                Target::Message => {  // CREATE MESSAGE
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
                Target::User => {  // CREATE USER
                    let email = request.email
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
                    let display_name = request.display_name
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'display_name' field"))?;
                    let password = request.password
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;
                    let public_key = request.public_key
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'public_key' field"))?;

                    // Salt and hash password
                    let password = Password::hash(&password, Option::None)?;

                    // Store user data
                    sqlx::query_file!("src/sql/create-user.sql",
                            email,
                            display_name,
                            public_key,
                            password.hash,
                            password.salt)
                        .execute(db_pool)
                        .await?;
                }
            }
        },
        Operation::Read => {
            match request.target {
                Target::Message => {  // READ MESSAGE
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
                Target::User => {  // READ USER
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Update => {
            match request.target {
                Target::Message => {  // UPDATE MESSAGE
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
                Target::User => {  // UPDATE USER
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Delete => {
            match request.target {
                Target::Message => {  // DELETE MESSAGE
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
                Target::User => {  // DELETE USER
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
    };

    Ok(())
}
