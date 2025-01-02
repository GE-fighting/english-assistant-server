use super::interface::SemesterService;
use crate::models::response::ApiResponse;
use crate::models::semester::Semester;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct SemesterServiceImpl {
    pool: PgPool,
}

impl SemesterServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SemesterService for SemesterServiceImpl {
    async fn get_semesters(&self) -> ApiResponse<Vec<Semester>> {
        match sqlx::query_as!(Semester, "SELECT * FROM semesters")
            .fetch_all(&self.pool)
            .await
        {
            Ok(semesters) => ApiResponse::success(semesters),
            Err(err) => ApiResponse::error(500, format!("Error fetching semesters: {}", err)),
        }
    }
}
