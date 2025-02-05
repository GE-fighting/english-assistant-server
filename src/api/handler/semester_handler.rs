use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::interfaces::semester_service::SemesterService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct SemesterHandler {
    service: Arc<dyn SemesterService>,
}

impl SemesterHandler {
    pub fn new(service: Arc<dyn SemesterService>) -> Self {
        Self { service }
    }
}

async fn get_semesters(data: web::Data<SemesterHandler>) -> impl Responder {
    let result = data.service.get_semesters().await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    SemesterHandler,
    get "/list" => get_semesters,
);
