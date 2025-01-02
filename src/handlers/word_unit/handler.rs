use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::dto::unit_word_dto::UnitWordDTO;
use crate::models::response::ApiResponse;
use crate::services::core::education::word_unit::{WordUnitService, WordUnitServiceImpl};

pub struct WordUnitHandler {
    service: WordUnitServiceImpl,
}

impl WordUnitHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: WordUnitServiceImpl::new(pool),
        }
    }

    pub async fn get_unit_words(&self, unit_id: i32) -> impl Responder {
        let result = self.service.get_unit_words(unit_id).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn create_unit_word(&self, unit_word: web::Json<UnitWordDTO>) -> impl Responder {
        let result = match self.service.create_word_unit_mapping(&unit_word).await {
            Ok(data) => ApiResponse::success(data),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
        HttpResponse::Ok().json(result)
    }

    pub async fn delete_unit_word(&self, id: i32) -> impl Responder {
        let result = match self.service.delete_unit_word(id).await {
            Ok(_) => ApiResponse::success(()),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
        HttpResponse::Ok().json(result)
    }
}
