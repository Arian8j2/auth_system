use std::fs::File;

use crate::{
    db::{establish_connection, setup, DbPool},
    utils::random::generate_random_six_digit_code,
};
use anyhow::Result;

pub async fn create_test_db() -> DbPool {
    let db_file = format!("/tmp/testdb_{}", generate_random_six_digit_code());
    File::create(&db_file).unwrap();

    let db_url = format!("sqlite://{db_file}");
    let pool = establish_connection(&db_url).await.unwrap();
    reset_db(&pool).await.unwrap();
    setup(&pool).await.unwrap();
    pool
}

async fn reset_db(pool: &DbPool) -> Result<()> {
    sqlx::query!("DROP TABLE IF EXISTS users; DROP TABLE IF EXISTS email_codes")
        .execute(pool)
        .await?;

    Ok(())
}
