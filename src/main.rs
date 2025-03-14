mod api;
mod config;
mod domain;
mod middleware;
mod repository;
mod service;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = config::database::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::auth::AuthMiddleware::default())
            .configure(api::config_services)
            .configure(api::swagger::config_swagger)
    })
        .build("127.0.0.1:8000")?
        .run()
        .await
}
