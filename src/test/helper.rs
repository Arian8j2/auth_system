use crate::db::{establish_connection, setup, DbPool};
use dotenv::dotenv;

pub async fn create_test_db() -> DbPool {
    dotenv().ok();
    let pool = establish_connection().await.unwrap();
    sqlx::query!("DROP TABLE IF EXISTS users")
        .execute(&pool)
        .await
        .unwrap();

    setup(&pool).await.unwrap();
    pool
}
