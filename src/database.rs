use crate::settings;

use std::env;
use std::error::Error;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

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
        sqlx::query_file!("sql/tables/drop.sql")
            .execute(&pool)
            .await?;
        println!("Existing tables dropped");
    }

    // Create tables if requested
    if settings::is_enabled("CREATE_DATABASE") {
        sqlx::query_file!("sql/tables/users.sql")
            .execute(&pool)
            .await?;

        sqlx::query_file!("sql/tables/conversations.sql")
            .execute(&pool)
            .await?;

        sqlx::query_file!("sql/tables/messages.sql")
            .execute(&pool)
            .await?;

        println!("New tables created");
    }

    Ok(pool)
}