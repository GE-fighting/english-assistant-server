use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/greet").route(web::get().to(handlers::greet)))
            // Add more routes here
    );
}
