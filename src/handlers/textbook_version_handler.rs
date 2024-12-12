use crate::models::entity::textbook_version;
use crate::models::response::ApiResponse;
use crate::services::textbook_version_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_textbook_versions(pool: web::Data<PgPool>) -> impl Responder {
    let result = textbook_version_service::get_textbook_versions(&pool).await;
    HttpResponse::Created().json(result)
}

pub async fn create_textbook_version(
    pool: web::Data<PgPool>,
    textbook_version: web::Json<textbook_version::TextbookVersion>,
) -> impl Responder {
    let result =
        match textbook_version_service::create_textbook_version(&pool, &textbook_version).await {
            Ok(textbook_version) => ApiResponse::success(textbook_version),
            Err(err) => ApiResponse::error_default_code(err.to_string()),
        };
    HttpResponse::Created().json(result)
}

pub async fn update_textbook_version(
    pool: web::Data<PgPool>,
    textbook_version: web::Json<textbook_version::TextbookVersion>,
) -> impl Responder {
    let result =
        match textbook_version_service::update_textbook_version(&pool, &textbook_version).await {
            Ok(textbook_version) => ApiResponse::success(textbook_version),
            Err(err) => ApiResponse::error_default_code(err.to_string()),
        };
    HttpResponse::Created().json(result)
}

pub async fn delete_textbook_version(
    pool: web::Data<PgPool>,
    textbook_version: web::Json<textbook_version::TextbookVersion>,
) -> impl Responder {
    let result = match textbook_version_service::delete_textbook_version(
        &pool,
        textbook_version.id.unwrap(),
    )
    .await
    {
        Ok(_) => ApiResponse::success("Success"),
        Err(err) => ApiResponse::error_default_code(err.to_string()),
    };
    HttpResponse::Created().json(result)
}
