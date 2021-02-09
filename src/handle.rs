use crate::api::request::{Request, Operation, Target};
use crate::api::response::Response;
use crate::auth;

use std::error::Error;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::str;
use std::time;
use async_std::task;
use async_std::prelude::*;
use async_std::net::TcpStream;
use async_tls::TlsAcceptor;
use log::{error, info};
use sqlx::PgPool;


// Handle incoming connections from clients
pub async fn handle_connection(stream: TcpStream, acceptor: &TlsAcceptor, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let interval = time::Duration::from_millis(500);
    let address = stream.peer_addr()?;
    let mut user = auth::Login{
        email: None,
        is_authenticated: false,
    };

    // Perform TLS handshake
    let handshake = acceptor.accept(stream);
    let mut stream = handshake.await?;
    info!("Handshake successful");

    // Polling connection
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                let result = handle_request(&buffer[..n], &mut user, db_pool).await;
                
                if let Err(e) = &result {
                    error!("{}", e);
                }

                let response = format_response(result.ok());
                stream.write(response.as_bytes());
            },
            Err(_) => task::sleep(interval).await,
        }
    }

    stream.flush();
    info!("Disconnected {}", address);
    Ok(())
}

// Handle individual requests from clients
async fn handle_request(data: &[u8], user: &mut auth::Login, db_pool: &PgPool) -> Result<Response, Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?;
    let request = Request::from_json(data)?;

    // Identify type of request
    let response = match request.operation {
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

    Ok(response)
}

// Format a response from the input
fn format_response(response: Option<Response>) -> String {
    let response = match response {
        Some(r) => r,
        // If no input provided, use a default failure response
        None => Response{
            status: 0,
            conversations: None,
            messages: None,
            users: None,
        },
    };

    response.to_json()
}