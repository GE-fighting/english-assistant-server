use crate::api::dto::textbook_dto::TextbookDTO;
use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::models::textbook::Textbook;
use crate::domain::services::interfaces::textbook_service::TextbookService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct TextbookHandler {
    service: Arc<dyn TextbookService>,
}

impl TextbookHandler {
    pub fn new(service: Arc<dyn TextbookService>) -> Self {
        Self { service }
    }
}

async fn get_textbooks(data: web::Data<TextbookHandler>) -> impl Responder {
    let result = data.service.get_textbooks().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn create_textbook(
    data: web::Data<TextbookHandler>,
    mut textbook: web::Json<Textbook>,
) -> impl Responder {
    let result = data.service.create_textbook(&mut textbook).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn delete_textbook(
    data: web::Data<TextbookHandler>,
    textbook_dto: web::Json<TextbookDTO>,
) -> impl Responder {
    let result = data.service.delete_textbook(&textbook_dto).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn get_unit_by_textbook(
    data: web::Data<TextbookHandler>,
    textbook_dto: web::Json<TextbookDTO>,
) -> impl Responder {
    let result = data.service.get_unit_by_textbook(&textbook_dto).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    TextbookHandler,
    get "/list" => get_textbooks,
    post "/create" => create_textbook,
    post "/delete" => delete_textbook,
    post "/units" => get_unit_by_textbook,
);
