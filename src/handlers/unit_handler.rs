use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;
use crate::services::unit_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_unit(pool: web::Data<PgPool>, unit: web::Json<UnitDTO>) -> impl Responder {
    log::info!("调用 /api/units/create, 创建单元");
    let result = unit_service::create_unit(&pool, &unit).await;
    HttpResponse::Created().json(result)
}

pub async fn get_units(pool: web::Data<PgPool>, unit_dto: web::Json<UnitDTO>) -> impl Responder {
    log::info!("调用 /api/units/get, 获取单元列表");
    let result = unit_service::get_unit(&pool, &unit_dto).await;
    HttpResponse::Created().json(result)
}

pub async fn delete_unit(pool: web::Data<PgPool>, unit_dto: web::Json<UnitDTO>) -> impl Responder {
    log::info!("调用 /api/units/delete, 删除单元");
    let result = match unit_service::delete_unit(&pool, &unit_dto).await {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(500, e.to_string()),
    };
    HttpResponse::Created().json(result)
}
