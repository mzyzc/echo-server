use std::env;
use std::error::Error;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    // Create tables if requested
    if let Ok(b) = env::var("INITIALIZE_DATABASE") {
        if b == "1" {
        /*
        sqlx::query_file!("sql/create-users.sql")
            .execute(&pool)
            .await?;
        */

        sqlx::query_file!("sql/create-conversations.sql")
            .execute(&pool)
            .await?;

        sqlx::query_file!("sql/create-messages.sql")
            .execute(&pool)
            .await?;
        }
    }

    Ok(pool)
}