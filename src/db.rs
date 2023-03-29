pub mod user;
pub mod smscodes;

use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub type DbPool = SqlitePool;

pub async fn establish_connection() -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("environment variable DATABASE_URL is not set!"))
        .await?;

    Ok(pool)
}

pub async fn setup(pool: &DbPool) -> Result<()> {
    user::setup_user_table(pool).await?;
    smscodes::setup_smscodes(pool).await
}
