use super::DbPool;
use crate::error::{ApiError, ApiResult};
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct EmailCode {
    pub code: u32,
    pub sent_date: DateTime<Utc>,
}

pub async fn insert_or_update_email_code(
    pool: &DbPool,
    email_address: &str,
    code: u32,
) -> ApiResult<()> {
    let now_date = Utc::now().to_rfc3339();
    let insert_result = sqlx::query!(
        "INSERT OR IGNORE INTO email_codes (email_address, last_sent_code, last_sent_date) VALUES (?, ?, ?)",
        email_address, code, now_date
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    if insert_result.rows_affected() == 0 {
        update_email_code(pool, email_address, code).await?;
    }

    Ok(())
}

async fn update_email_code(pool: &DbPool, email_address: &str, new_code: u32) -> ApiResult<()> {
    let now_date = Utc::now().to_rfc3339();
    sqlx::query!(
        "UPDATE email_codes SET last_sent_code=?, last_sent_date=? WHERE email_address=?",
        new_code,
        now_date,
        email_address,
    )
    .execute(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;
    Ok(())
}

pub async fn get_last_sent_email_code(
    pool: &DbPool,
    email_address: &str,
) -> ApiResult<Option<EmailCode>> {
    let record = sqlx::query!(
        "SELECT last_sent_code, last_sent_date FROM email_codes WHERE email_address=? LIMIT 1",
        email_address
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::SqlError { msg: e.to_string() })?;

    Ok(record.map(|r| EmailCode {
        code: r.last_sent_code.try_into().unwrap(),
        sent_date: DateTime::parse_from_rfc3339(&r.last_sent_date)
            .unwrap()
            .try_into()
            .unwrap(),
    }))
}

#[cfg(test)]
mod tests {
    use crate::test::helper::create_test_db;

    use super::*;

    #[actix_web::test]
    async fn get_and_insert_and_update_email_code() {
        let db = create_test_db().await;
        let email_address = "arianmoadabb@gmail.com";

        assert!(get_last_sent_email_code(&db, email_address).await.unwrap().is_none());

        assert!(insert_or_update_email_code(&db, email_address, 123456)
            .await
            .is_ok());
        let last_sent_email_code = get_last_sent_email_code(&db, email_address).await.unwrap().unwrap();
        assert_eq!(last_sent_email_code.code, 123456);

        assert!(insert_or_update_email_code(&db, email_address, 789102)
            .await
            .is_ok());
        let last_sent_email_code = get_last_sent_email_code(&db, email_address).await.unwrap().unwrap();
        assert_eq!(last_sent_email_code.code, 789102);
    }
}
