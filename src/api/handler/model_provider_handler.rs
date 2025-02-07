use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::ModelProviderService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct ModelProviderHandler {
    service: Arc<dyn ModelProviderService>,
}

impl ModelProviderHandler {
    pub fn new(service: Arc<dyn ModelProviderService>) -> Self {
        Self { service }
    }
}

async fn get_all_providers(data: web::Data<ModelProviderHandler>) -> impl Responder {
    let result = data.service.get_all_providers().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    ModelProviderHandler,
    get "/list" => get_all_providers,
);
