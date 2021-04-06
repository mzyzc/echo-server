use std::env;
use std::net::SocketAddr;
use async_std::prelude::*;
use async_std::net::TcpListener;
use async_std::task;
use dotenv;
use log::{error, info};

use echo_server;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Prepare database
    let pool = echo_server::database::init_db().await
        .expect("Could not initialize database");

    // Choose a socket address
    let socket_addr: SocketAddr = format!("{}:{}",
            env::var("IP_ADDRESS").unwrap_or(String::from("[::]")),
            env::var("PORT_NUMBER").unwrap_or(String::from("63100")))
        .parse()
        .expect("Could not parse socket address");

    // Set up TLS
    let acceptor = echo_server::tls::get_acceptor().await
        .expect("Could not accept TLS handshake");

    // Listen for incoming connections
    let listener = TcpListener::bind(socket_addr).await?;
    let mut incoming = listener.incoming();

    info!("Listening on port {}", socket_addr.port());

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let acceptor = acceptor.clone();
        let pool = pool.clone();

        info!("Successful connection from {}", stream.peer_addr()?);

        task::spawn(async move {
            let result = echo_server::handle_connection(stream, &acceptor, &pool).await;

            if let Err(e) = result {
                error!("{}", e);
            }
        });
    }

    Ok(())
}
