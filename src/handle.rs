use crate::pass::Password;
use crate::request::{Request, Operation, Target};

use std::error::Error;
use std::str;
use sqlx::PgPool;

pub async fn parse_request(data: &[u8], db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;

    match request.operation {
        Operation::Create => {
            match request.target {
                Target::Message => {
                    println!("undefined");
                }
                Target::User => {
                    let password = match request.password {
                        Some(p) => Password::hash(&p)?,
                        None => return Err("Could not hash password".into()),
                    };

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
                    println!("undefined");
                }
                Target::User => {
                    println!("undefined");
                }
            }
        },
        Operation::Update => {
            match request.target {
                Target::Message => {
                    println!("undefined");
                }
                Target::User => {
                    println!("undefined");
                }
            }
        },
        Operation::Delete => {
            match request.target {
                Target::Message => {
                    println!("undefined");
                }
                Target::User => {
                    println!("undefined");
                }
            }
        },
    };

    Ok(())
}
