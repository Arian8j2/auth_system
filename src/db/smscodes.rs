use super::DbPool;
use crate::error::{ApiError, ApiResult};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::Executor;

#[derive(Debug, PartialEq)]
pub struct SmsCode {
    pub code: u32,
    pub sent_date: DateTime<Utc>,
}

pub async fn setup_smscodes(pool: &DbPool) -> Result<()> {
    pool.execute(
        "
            CREATE TABLE IF NOT EXISTS smscodes (
                phone_number VARCHAR(16) PRIMARY KEY NOT NULL,
                last_sent_code UNSIGNED INT NOT NULL,
                last_sent_date VARCHAR(32) NOT NULL
            )
        ",
    )
    .await?;
    Ok(())
}

pub async fn insert_or_update_smscode(
    pool: &DbPool,
    phone_number: &str,
    code: u32,
) -> ApiResult<()> {
    let now_date = Utc::now().to_rfc3339();
    let insert_result = sqlx::query!(
        "INSERT OR IGNORE INTO smscodes (phone_number, last_sent_code, last_sent_date) VALUES (?, ?, ?)",
        phone_number, code, now_date
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    if insert_result.rows_affected() == 0 {
        update_smscode(pool, phone_number, code).await?;
    }

    Ok(())
}

async fn update_smscode(pool: &DbPool, phone_number: &str, new_code: u32) -> ApiResult<()> {
    let now_date = Utc::now().to_rfc3339();
    sqlx::query!(
        "UPDATE smscodes SET last_sent_code=?, last_sent_date=? WHERE phone_number=?",
        new_code,
        now_date,
        phone_number,
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;
    Ok(())
}

pub async fn get_last_sent_smscode(pool: &DbPool, phone_number: &str) -> ApiResult<SmsCode> {
    let record = sqlx::query!(
        "SELECT last_sent_code, last_sent_date FROM smscodes WHERE phone_number=? LIMIT 1",
        phone_number
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    let smscode = SmsCode {
        code: record.last_sent_code.try_into().unwrap(),
        sent_date: DateTime::parse_from_rfc3339(&record.last_sent_date)
            .unwrap()
            .try_into()
            .unwrap(),
    };
    Ok(smscode)
}

#[cfg(test)]
mod tests {
    use crate::test::helper::create_test_db;

    use super::*;

    #[actix_web::test]
    async fn test_get_insert_update_smscode() {
        let db = create_test_db().await;
        let phone_number = "09227588267";

        assert!(get_last_sent_smscode(&db, phone_number).await.is_err());

        assert!(insert_or_update_smscode(&db, phone_number, 123456).await.is_ok());
        let last_sent_smscode = get_last_sent_smscode(&db, phone_number).await.unwrap();
        assert_eq!(last_sent_smscode.code, 123456);

        assert!(insert_or_update_smscode(&db, phone_number, 789102).await.is_ok());
        let last_sent_smscode = get_last_sent_smscode(&db, phone_number).await.unwrap();
        assert_eq!(last_sent_smscode.code, 789102);
    }
}
