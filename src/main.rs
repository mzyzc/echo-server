use std::thread;
use std::time;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:63100")?;
    println!("Listening on port 63100");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let address = stream.peer_addr().unwrap();
                println!("Successful connection from {}", address);
                thread::spawn(move || {
                    handle_client(stream);
                    println!("Disconnected {}", address);
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
    let mut buffer = [0; 1024];
    loop {
        let data = stream.read(&mut buffer);
        match data {
            Ok(d) => {
                if d == 0 {
                    break;
                } else {
                    println!("Data received: {}", d);
                }
            },
            Err(_) => thread::sleep(time::Duration::from_secs(5)),
        }
    }
    stream.shutdown(Shutdown::Both).unwrap();
}
