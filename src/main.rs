use std::thread;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:63100")?;
    println!("Listening on port 63100");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Successful connection from {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    loop {}
}
