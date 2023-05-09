use super::DbPool;
use crate::error::{ApiError, ApiResult};

pub async fn insert_user(
    pool: &DbPool,
    name: &str,
    password: &str,
    email_address: &str,
) -> ApiResult<()> {
    let result = sqlx::query!(
        "INSERT OR IGNORE INTO users (name, password, email_address) VALUES (?, ?, ?)",
        name,
        password,
        email_address
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
    email_address: &str,
    password: &str,
) -> ApiResult<bool> {
    let result = sqlx::query!(
        "SELECT name FROM users WHERE email_address=? AND password=? LIMIT 1",
        email_address,
        password
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    Ok(result.is_some())
}
