use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::TextbookVersionHandler;
use crate::models::entity::textbook_version::TextbookVersion;

pub struct TextbookVersionRoutes;

impl TextbookVersionRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<TextbookVersionHandler>) {
        cfg.service(
            web::scope("/textbook-version")
                .route("/list", web::get().to(Self::get_textbook_versions))
                .route("/create", web::post().to(Self::create_textbook_version))
                .route("/update", web::post().to(Self::update_textbook_version))
                .route("/delete", web::post().to(Self::delete_textbook_version))
                .app_data(handler),
        );
    }

    async fn get_textbook_versions(
        handler: web::Data<TextbookVersionHandler>,
    ) -> impl actix_web::Responder {
        handler.get_textbook_versions().await
    }

    async fn create_textbook_version(
        handler: web::Data<TextbookVersionHandler>,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl actix_web::Responder {
        handler.create_textbook_version(textbook_version).await
    }

    async fn update_textbook_version(
        handler: web::Data<TextbookVersionHandler>,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl actix_web::Responder {
        handler.update_textbook_version(textbook_version).await
    }

    async fn delete_textbook_version(
        handler: web::Data<TextbookVersionHandler>,
        textbook_version: web::Json<TextbookVersion>,
    ) -> impl actix_web::Responder {
        handler.delete_textbook_version(textbook_version).await
    }
}
