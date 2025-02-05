use crate::domain::models::grade::Grade;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GradeService: Send + Sync {
    async fn get_grades(&self) -> Result<Vec<Grade>>;
}
