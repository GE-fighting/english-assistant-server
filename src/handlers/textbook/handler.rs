use actix_web::{web, HttpResponse, Responder};

use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::response::ApiResponse;
use crate::models::textbook::Textbook;
use crate::services::core::education::textbook::{TextbookService, TextbookServiceImpl};

pub struct TextbookHandler {
    service: TextbookServiceImpl,
}

impl TextbookHandler {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            service: TextbookServiceImpl::new(pool),
        }
    }

    pub async fn get_textbooks(&self) -> impl Responder {
        let result = self.service.get_textbooks().await;
        HttpResponse::Ok().json(result)
    }

    pub async fn create_textbook(&self, textbook: web::Json<Textbook>) -> impl Responder {
        let result = self.service.create_textbook(&textbook).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn delete_textbook(&self, textbook_dto: web::Json<TextbookDTO>) -> impl Responder {
        let result = match self.service.delete_textbook(&textbook_dto).await {
            Ok(_) => ApiResponse::success(()),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
        HttpResponse::Ok().json(result)
    }

    pub async fn get_unit_by_textbook(
        &self,
        textbook_dto: web::Json<TextbookDTO>,
    ) -> impl Responder {
        log::info!("调用 /api/units/get_by_textbook, 获取单元列表");
        let result = match self.service.get_unit_by_textbook(&textbook_dto).await {
            Ok(data) => ApiResponse::success(data),
            Err(e) => ApiResponse::error(500, e.to_string()),
        };
        HttpResponse::Ok().json(result)
    }
}
