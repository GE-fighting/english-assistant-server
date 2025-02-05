use crate::domain::models::grade::Grade;
use crate::domain::services::interfaces::grade_service::GradeService;
use crate::infrastructure::database::repositories::GradeRepository;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GradeServiceImpl {
    repository: Arc<dyn GradeRepository>,
}

impl GradeServiceImpl {
    pub fn new(repository: Arc<dyn GradeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GradeService for GradeServiceImpl {
    async fn get_grades(&self) -> Result<Vec<Grade>> {
        self.repository.find_all().await
    }
}
