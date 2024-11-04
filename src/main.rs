use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
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
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::config)
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}
