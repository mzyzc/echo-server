use std::time;
use std::env;
use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // Import environmental variables
    dotenv::dotenv().ok();

    // Set up database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await.expect("Error: could not initialize database");

    init_db(&pool).await;

    // Listen for incoming connections
    let socket_addr = format!("{}:{}",
        env::var("IP_ADDRESS").unwrap(),
        env::var("PORT_NUMBER").unwrap());

    let listener = TcpListener::bind(socket_addr).await?;
    let mut incoming = listener.incoming();
    println!("Listening on port {}", env::var("IP_ADDRESS").unwrap());

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
}

async fn init_db(pool: &Pool<Postgres>) {
    sqlx::query_file!("sql/create-users.sql")
        .execute(pool)
        .await.expect("Error: could not execute query");

    sqlx::query_file!("sql/create-conversations.sql")
        .execute(pool)
        .await.expect("Error: could not execute query");

    sqlx::query_file!("sql/create-messages.sql")
        .execute(pool)
        .await.expect("Error: could not execute query");
}