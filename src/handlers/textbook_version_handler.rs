use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::services::textbook_version_service;

pub async fn get_textbook_versions(pool: web::Data<PgPool>) -> impl Responder {
    let  result =  textbook_version_service::get_textbook_versions(&pool).await;
    HttpResponse::Created().json(result)
}