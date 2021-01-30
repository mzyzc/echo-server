use crate::auth::Password;
use crate::api::{Request, Operation, Target};

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::str;
use sqlx::PgPool;

pub async fn handle_request(data: &[u8], db_pool: &PgPool, user: &mut Option<String>) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;

    // Identify type of request
    match request.operation {
        Operation::Verify => {
            match request.target {
                Target::Users => {  // VERIFY USER
                    let email = request.email
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
                    let remote_pass = request.password
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
                        true => { *user = Some(email) },
                        false => return Err(Box::new(ioErr::new(ioErrKind::PermissionDenied, "Invalid password"))),
                    };
                }
                _ => {
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Create => {
            match request.target {
                Target::Users => {  // CREATE USER
                    let email = request.email
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'email' field"))?;
                    let password = request.password
                        .ok_or_else(|| ioErr::new(ioErrKind::InvalidInput, "Missing 'password' field"))?;
                    let public_key = request.public_key
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
                }
                _ => {
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Read => {
            match request.target {
                _ => {
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Update => {
            match request.target {
                _ => {
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
        Operation::Delete => {
            match request.target {
                _ => {
                    return Err(Box::new(ioErr::new(ioErrKind::InvalidInput, "Invalid operation")));
                }
            }
        },
    };

    Ok(())
}
