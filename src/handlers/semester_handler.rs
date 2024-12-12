use crate::services::semester_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_semester(pool: web::Data<PgPool>) -> impl Responder {
    let result = semester_service::get_semester(&pool).await;
    HttpResponse::Created().json(result)
}
