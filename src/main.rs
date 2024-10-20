use actix_web::{web, App, HttpServer};
mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = config::Settings::new().expect("Failed to load settings");
    let db_pool = db::create_pool(&settings.database_url())
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}
