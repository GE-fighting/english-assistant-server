use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::interfaces::grade_service::GradeService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct GradeHandler {
    service: Arc<dyn GradeService>,
}

impl GradeHandler {
    pub fn new(service: Arc<dyn GradeService>) -> Self {
        Self { service }
    }
}

async fn get_grades(handler: web::Data<GradeHandler>) -> impl Responder {
    let result = handler.service.get_grades().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    GradeHandler,
    get "/list" => get_grades,
);
