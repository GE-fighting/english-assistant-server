use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
mod config;
mod db;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置详细的日志环境变量
    std::env::set_var("RUST_LOG", "debug,actix_web=info,sqlx=warn");

    // 初始化日志
    utils::logger::init_logger().expect("Failed to initialize logger");

    // 记录启动日志
    log::info!("应用正在启动");
    log::debug!("调试信息：正在加载配置");

    let settings = config::Settings::global();
    log::info!(
        "配置加载成功，服务器地址: {}:{}",
        settings.server.host,
        settings.server.port
    );

    // 初始化数据库连接池
    let pool = db::create_pool(&settings.database)
        .await
        .expect("Failed to create database pool");
    log::info!("数据库连接池初始化成功");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}
