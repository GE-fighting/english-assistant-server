use sqlx::PgPool;
use crate::models::response::ApiResponse;
use crate::models::TextbookVersion::TextbookVersion;

pub async fn get_textbook_versions(pool: &PgPool) -> ApiResponse<Vec<TextbookVersion>> {
   match sqlx::query_as!(
       TextbookVersion,"SELECT *  FROM textbook_versions"
   )
        .fetch_all(pool)
        .await
   {
       Ok(textbook_versions) => ApiResponse::success(textbook_versions),
       Err(err) => ApiResponse::error(500, format!("Error fetching textbook versions: {}", err)),
   }
}