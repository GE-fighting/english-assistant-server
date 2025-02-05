use crate::domain::models::semester::Semester;
use async_trait::async_trait;

use anyhow::Result;

#[async_trait]
pub trait SemesterService: Send + Sync {
    async fn get_semesters(&self) -> Result<Vec<Semester>>;
}
