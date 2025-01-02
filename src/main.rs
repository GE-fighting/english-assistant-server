use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::handlers::{
    grade::{GradeHandler, GradeRoutes},
    semester::{SemesterHandler, SemesterRoutes},
    textbook::{TextbookHandler, TextbookRoutes},
    textbook_version::{TextbookVersionHandler, TextbookVersionRoutes},
    unit::{UnitHandler, UnitRoutes},
    word::{WordHandler, WordRoutes},
    word_unit::WordUnitHandler,
};
use crate::handlers::word_unit::WordUnitRoutes;

mod config;
mod db;
mod handlers;
mod models;
mod repositories;
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

    // 创建所有处理器实例
    let grade_handler = web::Data::new(GradeHandler::new(pool.clone()));
    let semester_handler = web::Data::new(SemesterHandler::new(pool.clone()));
    let textbook_handler = web::Data::new(TextbookHandler::new(pool.clone()));
    let textbook_version_handler = web::Data::new(TextbookVersionHandler::new(pool.clone()));
    let unit_handler = web::Data::new(UnitHandler::new(pool.clone()));
    let word_handler = web::Data::new(WordHandler::new(pool.clone()));
    let word_unit_handler = web::Data::new(WordUnitHandler::new(pool.clone()));

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
            .app_data(word_unit_handler.clone())
            .service(
                web::scope("/api")
                    .configure(|cfg| {
                        // 配置所有路由
                        GradeRoutes::configure(cfg, grade_handler.clone());
                        SemesterRoutes::configure(cfg, semester_handler.clone());
                        TextbookRoutes::configure(cfg, textbook_handler.clone());
                        TextbookVersionRoutes::configure(cfg, textbook_version_handler.clone());
                        UnitRoutes::configure(cfg, unit_handler.clone());
                        WordRoutes::configure(cfg, word_handler.clone());
                        WordUnitRoutes::configure(cfg, word_unit_handler.clone());
                    })
            )
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}
