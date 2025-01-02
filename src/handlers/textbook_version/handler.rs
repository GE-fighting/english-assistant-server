use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::entity::textbook_version::TextbookVersion;
use crate::models::response::ApiResponse;
use crate::services::core::education::textbook_version::{TextBookVersionService, TextbookVersionServiceImpl};

pub struct TextbookVersionHandler {
    service: TextbookVersionServiceImpl,
}

impl TextbookVersionHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: TextbookVersionServiceImpl::new(pool),
        }
    }

    pub async fn get_textbook_versions(&self) -> impl Responder {
        let result = self.service.get_textbook_versions().await;
        HttpResponse::Ok().json(result)
    }

    pub async fn create_textbook_version(
        &self,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl Responder {
        let result = match self
            .service
            .create_textbook_version(&textbook_version)
            .await
        {
            Ok(version) => ApiResponse::success(version),
            Err(err) => ApiResponse::error_default_code(err.to_string()),
        };
        HttpResponse::Ok().json(result)
    }

    pub async fn update_textbook_version(
        &self,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl Responder {
        let result = match self
            .service
            .update_textbook_version(&textbook_version)
            .await
        {
            Ok(version) => ApiResponse::success(version),
            Err(err) => ApiResponse::error_default_code(err.to_string()),
        };
        HttpResponse::Ok().json(result)
    }

    pub async fn delete_textbook_version(
        &self,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl Responder {
        let result = match self
            .service
            .delete_textbook_version(textbook_version.id.unwrap())
            .await
        {
            Ok(_) => ApiResponse::success("Success"),
            Err(err) => ApiResponse::error_default_code(err.to_string()),
        };
        HttpResponse::Ok().json(result)
    }
}
