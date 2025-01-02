use crate::models::grade::Grade;
use crate::models::response::ApiResponse;
use async_trait::async_trait;
use sqlx::PgPool;

use super::interface::GradeService;

pub struct GradeServiceImpl {
    pool: PgPool,
}

impl GradeServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GradeService for GradeServiceImpl {
    async fn get_grades(&self) -> ApiResponse<Vec<Grade>> {
        match sqlx::query_as!(Grade, "SELECT * FROM grades")
            .fetch_all(&self.pool)
            .await
        {
            Ok(grades) => ApiResponse::success(grades),
            Err(err) => ApiResponse::error(500, format!("Error fetching grades: {}", err)),
        }
    }
}
