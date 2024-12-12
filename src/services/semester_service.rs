use crate::models::response::ApiResponse;
use crate::models::semester::Semester;
use sqlx::PgPool;

// get all semesters
pub async fn get_semester(pool: &PgPool) -> ApiResponse<Vec<Semester>> {
    match sqlx::query_as!(Semester, "SELECT *  FROM semesters")
        .fetch_all(pool)
        .await
    {
        Ok(semester) => ApiResponse::success(semester),
        Err(err) => ApiResponse::error(500, format!("Error fetching semesters : {}", err)),
    }
}
