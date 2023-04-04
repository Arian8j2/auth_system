use super::DbPool;
use crate::error::{ApiError, ApiResult};
use anyhow::Result;
use sqlx::Executor;

pub async fn setup_user_table(pool: &DbPool) -> Result<()> {
    pool.execute(
        "
            CREATE TABLE IF NOT EXISTS users (
                phone_number VARCHAR(16) PRIMARY KEY NOT NULL,
                name VARCHAR(32) NOT NULL,
                password VARCHAR(255) NOT NULL
            )
            ",
    )
    .await?;
    Ok(())
}

pub async fn insert_user(
    pool: &DbPool,
    name: &str,
    password: &str,
    phone_number: &str,
) -> ApiResult<()> {
    let result = sqlx::query!(
        "INSERT OR IGNORE INTO users (name, password, phone_number) VALUES (?, ?, ?)",
        name,
        password,
        phone_number
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    if result.rows_affected() == 0 {
        Err(ApiError::RegisterDuplicate)
    } else {
        Ok(())
    }
}

pub async fn does_user_exists(
    pool: &DbPool,
    phone_number: &str,
    password: &str,
) -> ApiResult<bool> {
    let result = sqlx::query!(
        "SELECT name FROM users WHERE phone_number=? AND password=? LIMIT 1",
        phone_number,
        password
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    Ok(result.is_some())
}
