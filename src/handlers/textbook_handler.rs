use crate::models::dto::textbook_dto;
use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::response::ApiResponse;
use crate::models::textbook;
use crate::services::textbook_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_textbooks(pool: web::Data<PgPool>) -> impl Responder {
    let result = textbook_service::get_textbooks(&pool).await;
    HttpResponse::Created().json(result)
}

//新建textbook
pub async fn create_textbook(
    pool: web::Data<PgPool>,
    textbook: web::Json<textbook::Textbook>,
) -> impl Responder {
    let result = textbook_service::create_textbook(&pool, &textbook).await;
    HttpResponse::Created().json(result)
}

//删除textbook
pub async fn delete_textbook(
    pool: web::Data<PgPool>,
    textbook_dto: web::Json<textbook_dto::TextbookDTO>,
) -> impl Responder {
    let result = match textbook_service::delete_textbook(&pool, &textbook_dto).await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(500, e.to_string()),
    };
    HttpResponse::Created().json(result)
}

pub async fn get_unit_by_textbook(
    pool: web::Data<PgPool>,
    textbook_dto: web::Json<TextbookDTO>,
) -> impl Responder {
    log::info!("调用 /api/units/get_by_textbook, 获取单元列表");
    let result = match textbook_service::get_unit_by_textbook(&pool, &textbook_dto).await {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(500, e.to_string()),
    };
    HttpResponse::Created().json(result)
}
