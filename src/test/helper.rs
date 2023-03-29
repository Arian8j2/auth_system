use crate::db::{establish_connection, setup, DbPool};
use anyhow::Result;
use dotenv::dotenv;

pub async fn create_test_db() -> DbPool {
    dotenv().ok();
    let pool = establish_connection().await.unwrap();
    reset_db(&pool).await.unwrap();
    setup(&pool).await.unwrap();
    pool
}

async fn reset_db(pool: &DbPool) -> Result<()> {
    sqlx::query!("DROP TABLE IF EXISTS users; DROP TABLE IF EXISTS smscodes")
        .execute(pool)
        .await?;

    Ok(())
}
