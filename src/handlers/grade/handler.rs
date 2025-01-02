use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::services::core::education::grade::{GradeService, GradeServiceImpl};

pub struct GradeHandler {
    service: GradeServiceImpl,
}

impl GradeHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: GradeServiceImpl::new(pool),
        }
    }

    pub async fn get_grades(&self) -> impl Responder {
        let result = self.service.get_grades().await;
        HttpResponse::Ok().json(result)
    }
}
