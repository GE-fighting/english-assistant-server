use crate::api::dto::model_dto::ModelDto;
use crate::api::dto::response::ApiResponse;
use crate::common::utils::response::to_api_response;
use crate::define_routes;
use crate::domain::services::interfaces::SystemConfigService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tracing::{debug, error, info, instrument};

pub struct SystemConfigHandler {
    service: Arc<dyn SystemConfigService>,
}

impl SystemConfigHandler {
    pub fn new(service: Arc<dyn SystemConfigService>) -> Self {
        Self { service }
    }
}

#[instrument(skip(data))]
async fn set_use_model(
    data: web::Data<SystemConfigHandler>,
    model_dto: web::Json<ModelDto>,
) -> impl Responder {
    let result = if let Some(name) = &model_dto.name {
        let service_result = data.service.set_use_model(name).await;
        if service_result.is_ok() {
            info!("Successfully set use model for name: {}", name);
        } else {
            error!(
                "Failed to set use model for name: {}, error: {:?}",
                name, service_result
            );
        }
        service_result
    } else {
        error!("Model name is required");
        Err(anyhow::anyhow!("Model name is required"))
    };

    let response = to_api_response(result);
    HttpResponse::Ok().json(response)
}

define_routes!(
    SystemConfigHandler,
    post "/model" => set_use_model,
);
