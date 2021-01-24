mod database;
mod handle;
mod pass;
mod request;

use std::time;
use std::env;
use std::net::SocketAddr;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use dotenv;
use sqlx::PgPool;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // Import environmental variables
    dotenv::dotenv().ok();

    // Prepare database
    let pool = database::init_db().await
        .expect("Could not initialize database");

    // Listen for incoming connections
    let socket_addr: SocketAddr = format!("{}:{}",
            env::var("IP_ADDRESS").unwrap_or(String::from("[::]")),
            env::var("PORT_NUMBER").unwrap_or(String::from("63100")))
        .parse()
        .expect("Could not parse socket address");

    let listener = TcpListener::bind(socket_addr).await?;
    let mut incoming = listener.incoming();
    println!("Listening on port {}", socket_addr.port());

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Successful connection from {}", stream.peer_addr()?);
        task::spawn(
        handle_client(stream, pool.clone()));
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream, db_pool: PgPool) {
    let mut buffer = [0; 1024];
    let interval = time::Duration::from_millis(500);
    let address = stream.peer_addr().unwrap();

    // Polling connection
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => { break; },
            Ok(n) => { let _ = handle::parse_request(&buffer[..n], &db_pool).await; },
            Err(_) => { task::sleep(interval).await; },
        }
    }
    println!("Disconnected {}", address);
}
