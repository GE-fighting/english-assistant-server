use crate::models::entity::textbook_version::TextbookVersion;
use crate::models::response::ApiResponse;
use crate::repositories::textbook_version_repository;
use crate::services::core::education::textbook_version::TextBookVersionService;
use async_trait::async_trait;
use sqlx::{Error, PgPool};

pub struct TextbookVersionServiceImpl {
    pool: PgPool,
}

impl TextbookVersionServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        TextbookVersionServiceImpl { pool }
    }
}

#[async_trait]
impl TextBookVersionService for TextbookVersionServiceImpl {
    async fn get_textbook_versions(&self) -> ApiResponse<Vec<TextbookVersion>> {
        match sqlx::query_as!(TextbookVersion, "SELECT *  FROM textbook_versions")
            .fetch_all(&self.pool)
            .await
        {
            Ok(textbook_versions) => ApiResponse::success(textbook_versions),
            Err(err) => {
                ApiResponse::error(500, format!("Error fetching textbook versions: {}", err))
            }
        }
    }

    async fn create_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion, sqlx::Error> {
        textbook_version_repository::create_textbook_version(&self.pool, textbook_version).await
    }

    async fn update_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion, Error> {
        textbook_version_repository::update_textbook_version(&self.pool, textbook_version).await
    }

    async fn delete_textbook_version(&self, id: i32) -> Result<(), Error> {
        textbook_version_repository::delete_textbook_version(&self.pool, id).await
    }
}
