use crate::settings;

use std::env;
use std::error::Error;
use log::info;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

// Set up a database for connections
pub async fn init_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
    // Connect to database
    let max_conns: u32 = env::var("MAX_DB_CONNECTIONS")
        .unwrap_or(String::from("150"))
        .parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    // Drop existing tables if requested
    if settings::is_enabled("DROP_DATABASE") {
        drop_tables(&pool).await?;
    }

    // Create tables if requested
    if settings::is_enabled("CREATE_DATABASE") {
        create_tables(&pool).await?;
    }

    Ok(pool)
}

async fn drop_tables(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    sqlx::query_file!("src/sql/tables/drop.sql")
        .execute(pool)
        .await?;

    info!("Existing tables dropped");
    Ok(())
}
async fn create_tables(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    sqlx::query_file!("src/sql/tables/users.sql")
        .execute(pool)
        .await?;

    sqlx::query_file!("src/sql/tables/conversations.sql")
        .execute(pool)
        .await?;

    sqlx::query_file!("src/sql/tables/participants.sql")
        .execute(pool)
        .await?;

    sqlx::query_file!("src/sql/tables/messages.sql")
        .execute(pool)
        .await?;

    info!("New tables created");
    Ok(())
}