use crate::models::response::ApiResponse;
use crate::models::semester::Semester;
use async_trait::async_trait;

#[async_trait]
pub trait SemesterService {
    async fn get_semesters(&self) -> ApiResponse<Vec<Semester>>;
}
