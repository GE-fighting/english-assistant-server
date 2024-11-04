use sqlx::PgPool;
use crate::models::ApiResponse;
use crate::models::grade::Grade;

pub async fn get_grades(pool: &PgPool) ->ApiResponse<Vec<Grade>>{
    match sqlx::query_as!(
       Grade,"SELECT *  FROM grades"
   )
        .fetch_all(pool)
        .await
    {
        Ok(grades) => ApiResponse::success(grades),
        Err(err) => ApiResponse::error(500, format!("Error fetching grades : {}", err)),
    }
}