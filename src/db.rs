pub mod email_codes;
pub mod user;

use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub type DbPool = SqlitePool;

pub async fn establish_connection(db_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    Ok(pool)
}

pub async fn setup(pool: &DbPool) -> Result<()> {
    sqlx::migrate!("./src/db/migrations").run(pool).await?;
    Ok(())
}
