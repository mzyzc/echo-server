use crate::auth::Password;
use crate::request::{Request, Operation, Target};

use std::error::Error;
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
                Target::User => {
                    // Retrieve credentials from database
                    let stream = sqlx::query_file!("sql/verify-user.sql", request.email)
                        .fetch_one(db_pool)
                        .await?;

                    let password = Password{hash: stream.pass, salt: stream.salt};

                    // Compare password to user input
                    let result = match request.password {
                        Some(p) => password.is_valid(&p)?,
                        None => return Err("Missing 'password' field".into()),
                    };
                    match result {
                        true => { user = Some(request.email.unwrap()) },
                        false => return Err("Invalid password".into()),
                    };
                }
                Target::Message => {
                    return Err("Invalid operation".into());
                }
            }
        },
        Operation::Create => {
            match request.target {
                Target::Message => {
                    return Err("Invalid operation".into());
                }
                Target::User => {
                    // Salt and hash password
                    let password = match request.password {
                        Some(p) => Password::without_salt(&p)?,
                        None => return Err("Missing 'password' field".into()),
                    };

                    // Store user data
                    sqlx::query_file!("sql/create-user.sql",
                            request.email,
                            request.display_name,
                            request.public_key,
                            password.hash,
                            password.salt)
                        .execute(db_pool)
                        .await?;
                }
            }
        },
        Operation::Read => {
            match request.target {
                Target::Message => {
                    return Err("Invalid operation".into());
                }
                Target::User => {
                    return Err("Invalid operation".into());
                }
            }
        },
        Operation::Update => {
            match request.target {
                Target::Message => {
                    return Err("Invalid operation".into());
                }
                Target::User => {
                    return Err("Invalid operation".into());
                }
            }
        },
        Operation::Delete => {
            match request.target {
                Target::Message => {
                    return Err("Invalid operation".into());
                }
                Target::User => {
                    return Err("Invalid operation".into());
                }
            }
        },
    };

    Ok(())
}
