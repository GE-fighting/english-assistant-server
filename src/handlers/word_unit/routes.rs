use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::WordUnitHandler;
use crate::models::dto::unit_word_dto::UnitWordDTO;

pub struct WordUnitRoutes;

impl WordUnitRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<WordUnitHandler>) {
        cfg.service(
            web::scope("/unit-word")
                .route("/words", web::post().to(Self::get_unit_words))
                .route("/create", web::post().to(Self::create_unit_word))
                .route("/delete", web::post().to(Self::delete_unit_word))
                .app_data(handler),
        );
    }

    async fn get_unit_words(
        handler: web::Data<WordUnitHandler>,
        unit_word: web::Json<UnitWordDTO>,
    ) -> impl actix_web::Responder {
        handler.get_unit_words(unit_word.unit_id.unwrap()).await
    }

    async fn create_unit_word(
        handler: web::Data<WordUnitHandler>,
        unit_word: web::Json<UnitWordDTO>,
    ) -> impl actix_web::Responder {
        handler.create_unit_word(unit_word).await
    }

    async fn delete_unit_word(
        handler: web::Data<WordUnitHandler>,
        unit_word: web::Json<UnitWordDTO>,
    ) -> impl actix_web::Responder {
        handler.delete_unit_word(unit_word.id.unwrap()).await
    }
}
