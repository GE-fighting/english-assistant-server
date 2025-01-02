use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;
use crate::services::core::education::unit::{UnitService, UnitServiceImpl};

pub struct UnitHandler {
    service: UnitServiceImpl,
}

impl UnitHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: UnitServiceImpl::new(pool),
        }
    }

    pub async fn create_unit(&self, unit: web::Json<UnitDTO>) -> impl Responder {
        let result = self.service.create_unit(&unit).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn get_units(&self, unit_dto: web::Json<UnitDTO>) -> impl Responder {
        let result = self.service.get_units(&unit_dto).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn delete_unit(&self, unit_dto: web::Json<UnitDTO>) -> impl Responder {
        let result = match self.service.delete_unit(&unit_dto).await {
            Ok(_) => ApiResponse::success(()),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
        HttpResponse::Ok().json(result)
    }
}
