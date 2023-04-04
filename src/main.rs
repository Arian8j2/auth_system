mod api;
mod db;
mod error;
mod smsprovider;
mod utils;

#[cfg(test)]
mod test;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use smsprovider::{SmsProvider, TempSmsProvider};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = db::establish_connection().await?;
    db::setup(&pool).await?;

    let sms_provider: Arc<dyn SmsProvider + Send + Sync> = Arc::new(TempSmsProvider::new());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::from(sms_provider.clone()))
            .service(api::register::register)
            .service(api::sendsmscode::sendsmscode)
            .service(api::login::login)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
