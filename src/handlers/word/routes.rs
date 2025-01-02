use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::WordHandler;
use crate::models::word::Word;

pub struct WordRoutes;

impl WordRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<WordHandler>) {
        cfg.service(
            web::scope("/word")
                .route("/create", web::post().to(Self::create_word))
                .route("/get", web::post().to(Self::get_word))
                .route("/update", web::post().to(Self::update_batch_words))
                .app_data(handler),
        );
    }

    async fn create_word(
        handler: web::Data<WordHandler>,
        word: web::Json<Word>,
    ) -> impl actix_web::Responder {
        handler.create_word(word).await
    }

    async fn get_word(
        handler: web::Data<WordHandler>,
        word: web::Json<Word>,
    ) -> impl actix_web::Responder {
        handler.get_word(word).await
    }

    async fn update_batch_words(handler: web::Data<WordHandler>) -> impl actix_web::Responder {
        handler.update_batch_words().await
    }
}
