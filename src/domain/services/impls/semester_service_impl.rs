use crate::domain::models::semester::Semester;
use crate::domain::services::interfaces::semester_service::SemesterService;
use crate::infrastructure::database::repositories::SemesterRepository;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct SemesterServiceImpl {
    repository: Arc<dyn SemesterRepository>,
}

impl SemesterServiceImpl {
    pub fn new(repository: Arc<dyn SemesterRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl SemesterService for SemesterServiceImpl {
    async fn get_semesters(&self) -> Result<Vec<Semester>> {
        self.repository.find_all().await
    }
}
