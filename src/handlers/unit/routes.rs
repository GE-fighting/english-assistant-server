use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::UnitHandler;
use crate::models::dto::unit_dto::UnitDTO;

pub struct UnitRoutes;

impl UnitRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<UnitHandler>) {
        cfg.service(
            web::scope("/unit")
                .route("/create", web::post().to(Self::create_unit))
                .route("/list", web::post().to(Self::get_units))
                .route("/delete", web::post().to(Self::delete_unit))
                .app_data(handler),
        );
    }

    async fn create_unit(
        handler: web::Data<UnitHandler>,
        unit: web::Json<UnitDTO>,
    ) -> impl actix_web::Responder {
        handler.create_unit(unit).await
    }

    async fn get_units(
        handler: web::Data<UnitHandler>,
        unit_dto: web::Json<UnitDTO>,
    ) -> impl actix_web::Responder {
        handler.get_units(unit_dto).await
    }

    async fn delete_unit(
        handler: web::Data<UnitHandler>,
        unit_dto: web::Json<UnitDTO>,
    ) -> impl actix_web::Responder {
        handler.delete_unit(unit_dto).await
    }
}
