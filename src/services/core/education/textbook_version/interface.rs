use crate::models::entity::textbook_version::TextbookVersion;
use crate::models::response::ApiResponse;
use async_trait::async_trait;

#[async_trait]
pub trait TextBookVersionService {
    /// Get all textbook versions
    async fn get_textbook_versions(&self) -> ApiResponse<Vec<TextbookVersion>>;

    /// Create a new textbook version
    async fn create_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion, sqlx::Error>;

    /// Update an existing textbook version
    async fn update_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion, sqlx::Error>;

    /// Delete a textbook version
    async fn delete_textbook_version(&self, id: i32) -> Result<(), sqlx::Error>;
}
