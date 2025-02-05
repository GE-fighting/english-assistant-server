use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::interfaces::word_unit_service::WordUnitService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use crate::api::dto::unit_word_dto::WordDTO;

pub struct WordUnitHandler {
    service: Arc<dyn WordUnitService>,
}

impl WordUnitHandler {
    pub fn new(service: Arc<dyn WordUnitService>) -> Self {
        Self { service }
    }
}

async fn get_unit_words(
    data: web::Data<WordUnitHandler>,
    unit_word: web::Json<WordDTO>,
) -> impl Responder {
    let result = data
        .service
        .get_unit_words(unit_word.unit_id.unwrap())
        .await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn create_word_unit_mapping(
    data: web::Data<WordUnitHandler>,
    unit_word: web::Json<WordDTO>,
) -> impl Responder {
    let result = data.service.create_word_unit_mapping(&unit_word).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn delete_unit_word(
    data: web::Data<WordUnitHandler>,
    unit_word: web::Json<WordDTO>,
) -> impl Responder {
    let result = data.service.delete_unit_word(unit_word.id.unwrap()).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    WordUnitHandler,
    post "/words" => get_unit_words,
    post "/create" => create_word_unit_mapping,
    post "/delete" => delete_unit_word,
);
