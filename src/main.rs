#[macro_use]
extern crate lazy_static;

mod api;
mod db;
mod email_sender;
mod error;
mod utils;

#[cfg(test)]
mod test;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use email_sender::{EmailSender, RealEmailSender};
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("environment variable DATABASE_URL is not set!");
    let pool = db::establish_connection(&db_url).await?;
    db::setup(&pool).await?;

    let email_provider: Arc<dyn EmailSender + Send + Sync> = Arc::new(RealEmailSender::new());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::from(email_provider.clone()))
            .service(api::register::register)
            .service(api::send_email_code::send_email_code)
            .service(api::login::login)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
