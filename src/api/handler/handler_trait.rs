use actix_web::web;

pub trait Handler {
    fn configure(cfg: &mut web::ServiceConfig);
    fn register(cfg: &mut web::ServiceConfig) {
        Self::configure(cfg);
    }
}
