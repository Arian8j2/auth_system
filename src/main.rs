mod api;
mod db;
mod error;

#[cfg(test)]
mod test;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = db::establish_connection().await?;
    db::setup(&pool).await?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(api::register::register)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
