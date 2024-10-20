use crate::models::Word;
use crate::services::word_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_word(pool: web::Data<PgPool>, word: web::Json<Word>) -> impl Responder {
    let result = word_service::create_word(&pool, &word.word).await;
    HttpResponse::Created().json(result)
}

pub async fn get_word(pool: web::Data<PgPool>, word: web::Json<Word>) -> impl Responder {
    let result = word_service::get_word(&pool, &word.word).await;
    HttpResponse::Created().json(result)
}

pub async fn update_batch_words(pool: web::Data<PgPool>) -> impl Responder {
    let result = word_service::update_batch_words(&pool).await;
    HttpResponse::Created().json(result)
}
