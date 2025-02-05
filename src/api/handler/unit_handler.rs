use crate::api::dto::unit_dto::UnitDTO;
use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::interfaces::unit_service::UnitService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub struct UnitHandler {
    service: Arc<dyn UnitService>,
}

impl UnitHandler {
    pub fn new(service: Arc<dyn UnitService>) -> Self {
        Self { service }
    }
}

async fn create_unit(data: web::Data<UnitHandler>, unit: web::Json<UnitDTO>) -> impl Responder {
    let result = data.service.create_unit(&unit).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn get_units(data: web::Data<UnitHandler>, unit_dto: web::Json<UnitDTO>) -> impl Responder {
    let result = data.service.get_units(&unit_dto).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

async fn delete_unit(data: web::Data<UnitHandler>, unit_dto: web::Json<UnitDTO>) -> impl Responder {
    let result = data.service.delete_unit(unit_dto.id.unwrap()).await;
    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    UnitHandler,
    post "/create" => create_unit,
    post "/list" => get_units,
    post "/delete" => delete_unit,
);
