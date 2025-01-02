use actix_web::web;
use actix_web::web::ServiceConfig;

use super::handler::SemesterHandler;

pub struct SemesterRoutes;

impl SemesterRoutes {
    pub fn configure(cfg: &mut ServiceConfig, handler: web::Data<SemesterHandler>) {
        cfg.service(
            web::scope("/semester")
                .route("/list", web::get().to(Self::get_semesters))
                .app_data(handler),
        );
    }

    async fn get_semesters(handler: web::Data<SemesterHandler>) -> impl actix_web::Responder {
        handler.get_semesters().await
    }
}
