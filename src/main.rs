use std::time;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:63100").await?;
    let mut incoming = listener.incoming();
    println!("Listening on port 63100");

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let address = stream.peer_addr().unwrap();
        println!("Successful connection from {}", address);
        handle_client(stream).await;
        println!("Disconnected {}", address);
    }
    Ok(())
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
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
            Err(_) => task::sleep(time::Duration::from_millis(500)).await,
        }
    }
}
