use crate::models::entity::textbook_version::TextbookVersion;
use crate::models::response::ApiResponse;
use crate::repositories::textbook_version_repository;
use sqlx::PgPool;

pub async fn get_textbook_versions(pool: &PgPool) -> ApiResponse<Vec<TextbookVersion>> {
    match sqlx::query_as!(TextbookVersion, "SELECT *  FROM textbook_versions")
        .fetch_all(pool)
        .await
    {
        Ok(textbook_versions) => ApiResponse::success(textbook_versions),
        Err(err) => ApiResponse::error(500, format!("Error fetching textbook versions: {}", err)),
    }
}

pub async fn create_textbook_version(
    pool: &PgPool,
    textbook_version: &TextbookVersion,
) -> Result<TextbookVersion, sqlx::Error> {
    textbook_version_repository::create_textbook_version(pool, textbook_version).await
}

pub async fn update_textbook_version(
    pool: &PgPool,
    textbook_version: &TextbookVersion,
) -> Result<TextbookVersion, sqlx::Error> {
    textbook_version_repository::update_textbook_version(pool, textbook_version).await
}

pub async fn delete_textbook_version(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    textbook_version_repository::delete_textbook_version(pool, id).await
}
