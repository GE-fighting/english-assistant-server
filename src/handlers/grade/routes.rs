use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::GradeHandler;

pub struct GradeRoutes;

impl GradeRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<GradeHandler>) {
        cfg.service(
            web::scope("/grade")
                .route("/list", web::get().to(Self::get_grades))
                .app_data(handler),
        );
    }

    async fn get_grades(handler: web::Data<GradeHandler>) -> impl actix_web::Responder {
        handler.get_grades().await
    }
}
