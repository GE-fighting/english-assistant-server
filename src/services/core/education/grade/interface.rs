use crate::models::grade::Grade;
use crate::models::response::ApiResponse;
use async_trait::async_trait;

#[async_trait]
pub trait GradeService {
    async fn get_grades(&self) -> ApiResponse<Vec<Grade>>;
}
