mod database;

use std::time;
use std::env;
use std::net::SocketAddr;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use dotenv;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // Import environmental variables
    dotenv::dotenv().ok();

    // Prepare database
    let _pool = database::init_db().await.expect("Could not initialize database");

    // Listen for incoming connections
    let socket_addr: SocketAddr = format!("{}:{}",
            env::var("IP_ADDRESS").unwrap_or(String::from("[::]")),
            env::var("PORT_NUMBER").unwrap_or(String::from("63100")))
        .parse().expect("Could not parse socket address");

    let listener = TcpListener::bind(socket_addr).await?;
    let mut incoming = listener.incoming();
    println!("Listening on port {}", socket_addr.port());

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Successful connection from {}", stream.peer_addr()?);
        task::spawn(handle_client(stream));
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let interval = time::Duration::from_millis(500);

    // Polling connection
    loop {
        let data = stream.read(&mut buffer).await;
        match data {
            Ok(d) => {
                if d == 0 {
                    break;
                } else {
                    println!("Data received");
                }
            },
            Err(_) => task::sleep(interval).await,
        }
    }
    println!("Disconnected {}", stream.peer_addr().unwrap());
}
