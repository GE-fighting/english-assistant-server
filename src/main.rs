use std::fmt::Write;
use crate::api::configure_routes;
use crate::app::{HandlerFactory, RequestLogger, ServiceContainer};
use crate::common::utils;
use crate::config::Settings;
use crate::infrastructure::cache::redis;
use crate::infrastructure::database::db;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use chrono::{DateTime, Utc, TimeZone, FixedOffset, Duration};
use chrono_tz::Asia::Shanghai;
use tracing_appender::rolling;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::infrastructure::llm::init_llm_manager;

mod api;
mod app;
mod common;
mod config;
mod domain;
mod infrastructure;

struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = Utc::now() + Duration::hours(8);
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f %z"))
    }
}


async fn initialize_infrastructure() -> std::io::Result<Arc<PgPool>> {
    // Initialize environment variables
    dotenv().ok();

    // Initialize tracing subscriber
    let file_appender = rolling::daily("logs", "app.log");

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()) //使用环境变量控制日志级别
        .with_timer(CustomTimer)
        .with_writer(file_appender)
        .with_ansi(false)
        .compact()  // 使用简洁的输出格式
        // .with_target(false) // 不显示 target
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Load settings
    let settings = Settings::global();
    info!(
        "Configuration loaded successfully, server address: {}:{}",
        settings.app.server.host,
        settings.app.server.port
    );

    // Initialize database connection pool
    let pool = Arc::new(
        db::create_pool(&settings.database)
            .await
            .expect("Failed to create database pool"),
    );
    info!("Database connection pool initialized successfully");

    // Initialize redis connection
    redis::init().await.expect("Failed to connect to Redis");
    info!("Redis connection initialized successfully");

    Ok(pool)
}

fn create_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize infrastructure
    let pool = initialize_infrastructure().await?;

    // Initialize dependency injection container
    let service_container = Arc::new(ServiceContainer::new(pool));

    // init llm
    init_llm_manager();
    // Initialize handler factory
    let handler_factory = HandlerFactory::new(
        service_container.get_grade_service(),
        service_container.get_semester_service(),
        service_container.get_system_config_service(),
        service_container.get_textbook_service(),
        service_container.get_textbook_version_service(),
        service_container.get_unit_service(),
        service_container.get_word_service(),
        service_container.get_word_unit_service(),
        service_container.get_model_provider_service(),
    );

    let settings = Settings::global();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(create_cors())
            .wrap(middleware::Logger::default())
            .wrap(RequestLogger)
            .configure(|cfg| configure_routes(cfg, handler_factory.clone()))
    })
    .bind(format!(
        "{}:{}",
        settings.app.server.host, settings.app.server.port
    ))?
    .run()
    .await
}
