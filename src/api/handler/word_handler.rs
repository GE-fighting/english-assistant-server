use crate::api::dto::unit_word_dto::WordPageRequestDTO;
use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::models::word::Word;
use crate::domain::services::interfaces::word_service::WordService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tracing::instrument;

pub struct WordHandler {
    service: Arc<dyn WordService>,
}

impl WordHandler {
    pub fn new(service: Arc<dyn WordService>) -> Self {
        Self { service }
    }
}

async fn create_word(data: web::Data<WordHandler>, word: web::Json<Word>) -> impl Responder {
    let result = data.service.create_word(&word.word).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn get_word(data: web::Data<WordHandler>, word: web::Json<Word>) -> impl Responder {
    let result = data.service.get_word(&word.word).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn update_batch_words(data: web::Data<WordHandler>) -> impl Responder {
    let result = data.service.update_batch_words().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}



define_routes!(
    WordHandler,
    post "/create" => create_word,
    post "/get" => get_word,
    post "/update-batch" => update_batch_words,
);
