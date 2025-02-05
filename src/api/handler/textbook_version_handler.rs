use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::models::textbook_version::TextbookVersion;
use crate::domain::services::interfaces::textbook_version_service::TextbookVersionService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct TextbookVersionHandler {
    service: Arc<dyn TextbookVersionService>,
}

impl TextbookVersionHandler {
    pub fn new(service: Arc<dyn TextbookVersionService>) -> Self {
        Self { service }
    }
}

async fn get_textbook_versions(data: web::Data<TextbookVersionHandler>) -> impl Responder {
    let result = data.service.get_textbook_versions().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn create_textbook_version(
    data: web::Data<TextbookVersionHandler>,
    textbook_version: web::Json<TextbookVersion>,
) -> impl Responder {
    let result = data
        .service
        .create_textbook_version(&textbook_version)
        .await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn update_textbook_version(
    data: web::Data<TextbookVersionHandler>,
    textbook_version: web::Json<TextbookVersion>,
) -> impl Responder {
    let result = data
        .service
        .update_textbook_version(&textbook_version)
        .await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn delete_textbook_version(
    data: web::Data<TextbookVersionHandler>,
    textbook_version: web::Json<TextbookVersion>,
) -> impl Responder {
    let result = data
        .service
        .delete_textbook_version(textbook_version.id.unwrap())
        .await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    TextbookVersionHandler,
    get "/list" => get_textbook_versions,
    post "/create" => create_textbook_version,
    post "/update" => update_textbook_version,
    post "/delete" => delete_textbook_version,
);
