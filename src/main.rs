mod api;
mod auth;
mod database;
mod handle;
mod settings;
mod tls;

use std::time;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tls::TlsAcceptor;
use dotenv;
use log::{error, info};
use sqlx::PgPool;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Prepare database
    let pool = database::init_db().await
        .expect("Could not initialize database");

    // Listen for incoming connections
    let socket_addr: SocketAddr = format!("{}:{}",
            env::var("IP_ADDRESS").unwrap_or(String::from("[::]")),
            env::var("PORT_NUMBER").unwrap_or(String::from("63100")))
        .parse()
        .expect("Could not parse socket address");

    let acceptor = tls::get_acceptor().await
        .expect("Could not accept TLS handshake");

    let listener = TcpListener::bind(socket_addr).await?;
    let mut incoming = listener.incoming();
    info!("Listening on port {}", socket_addr.port());

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let acceptor = acceptor.clone();
        let pool = pool.clone();
        info!("Successful connection from {}", stream.peer_addr()?);
        task::spawn(async move {
            let result = handle_connection(stream, &acceptor, &pool).await;
            if let Err(e) = result {
                error!("{}", e);
            }
        });
    }

    Ok(())
}

// Handle incoming connections from clients
async fn handle_connection(stream: TcpStream, acceptor: &TlsAcceptor, db_pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let interval = time::Duration::from_millis(500);
    let address = stream.peer_addr().unwrap();
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
                let result = handle::handle_request(&buffer[..n], &mut user, db_pool).await;
                let response = handle::format_response(result.ok());
                stream.write(response.as_bytes());
            },
            Err(_) => task::sleep(interval).await,
        }
    }

    stream.flush();
    info!("Disconnected {}", address);
    Ok(())
}
