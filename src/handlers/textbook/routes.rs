use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::TextbookHandler;
use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::textbook::Textbook;

pub struct TextbookRoutes;

impl TextbookRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<TextbookHandler>) {
        cfg.service(
            web::scope("/textbook")
                .route("/list", web::get().to(Self::get_textbooks))
                .route("/create", web::post().to(Self::create_textbook))
                .route("/delete", web::post().to(Self::delete_textbook))
                .route("/units", web::post().to(Self::get_unit_by_textbook))
                .app_data(handler),
        );
    }

    // 路由处理函数
    async fn get_textbooks(handler: web::Data<TextbookHandler>) -> impl actix_web::Responder {
        handler.get_textbooks().await
    }

    async fn create_textbook(
        handler: web::Data<TextbookHandler>,
        textbook: web::Json<Textbook>,
    ) -> impl actix_web::Responder {
        handler.create_textbook(textbook).await
    }

    async fn delete_textbook(
        handler: web::Data<TextbookHandler>,
        textbook_dto: web::Json<TextbookDTO>,
    ) -> impl actix_web::Responder {
        handler.delete_textbook(textbook_dto).await
    }

    async fn get_unit_by_textbook(
        handler: web::Data<TextbookHandler>,
        textbook_dto: web::Json<TextbookDTO>,
    ) -> impl actix_web::Responder {
        handler.get_unit_by_textbook(textbook_dto).await
    }
}
