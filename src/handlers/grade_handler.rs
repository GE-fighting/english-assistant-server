use crate::services::grade_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_grades(pool: web::Data<PgPool>) -> impl Responder {
    let result = grade_service::get_grades(&pool).await;
    HttpResponse::Created().json(result)
}
