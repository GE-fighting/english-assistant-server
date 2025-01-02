use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::services::core::education::semester::{SemesterService, SemesterServiceImpl};

pub struct SemesterHandler {
    service: SemesterServiceImpl,
}

impl SemesterHandler {
    pub fn new(pool: PgPool) -> Self {
        Self {
            service: SemesterServiceImpl::new(pool),
        }
    }

    pub async fn get_semesters(&self) -> impl Responder {
        let result = self.service.get_semesters().await;
        HttpResponse::Ok().json(result)
    }
}
