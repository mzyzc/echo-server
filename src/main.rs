use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:63100")?;
    println!("Listening on port 63100");

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    println!("Successful connection from {}", stream.peer_addr().unwrap().to_string());
}
