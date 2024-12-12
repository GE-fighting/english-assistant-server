use crate::models::dto::unit_word_dto::UnitWordDTO;
use crate::models::response::ApiResponse;
use crate::services::word_unit_mapping_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_unit_word(
    pool: web::Data<PgPool>,
    unit_word: web::Json<UnitWordDTO>,
) -> impl Responder {
    let result = match word_unit_mapping_service::create_word_unit_mapping(&pool, &unit_word).await
    {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(500, e.to_string()),
    };
    HttpResponse::Created().json(result)
}

pub async fn get_unit_words(
    pool: web::Data<PgPool>,
    unit_word: web::Json<UnitWordDTO>,
) -> impl Responder {
    let result = word_unit_mapping_service::get_unit_words(&pool, unit_word.unit_id.unwrap()).await;
    HttpResponse::Ok().json(result)
}

pub async fn delete_unit_word(
    pool: web::Data<PgPool>,
    unit_word: web::Json<UnitWordDTO>,
) -> impl Responder {
    let result =
        match word_unit_mapping_service::delete_unit_word(&pool, unit_word.id.unwrap()).await {
            Ok(_) => ApiResponse::success(()),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
    HttpResponse::Ok().json(result)
}
