use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
mod config;
mod routes;
mod db;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // 加载 .env 文件
    let settings = config::Settings::new().expect("Failed to load settings");
    let db_pool = db::create_pool(&settings.database_url()).await.expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}
