use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::word::Word;
use crate::services::core::education::word::{WordService, WordServiceImpl};

pub struct WordHandler {
    service: WordServiceImpl,
}

impl WordHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: WordServiceImpl::new(pool),
        }
    }

    pub async fn create_word(&self, word: web::Json<Word>) -> impl Responder {
        let result = self.service.create_word(&word.word).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn get_word(&self, word: web::Json<Word>) -> impl Responder {
        let result = self.service.get_word(&word.word).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn update_batch_words(&self) -> impl Responder {
        let result = self.service.update_batch_words().await;
        HttpResponse::Ok().json(result)
    }
}
