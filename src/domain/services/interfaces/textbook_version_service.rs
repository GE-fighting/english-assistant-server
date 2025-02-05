use crate::domain::models::textbook_version::TextbookVersion;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TextbookVersionService: Send + Sync {
    /// Get all textbook versions
    async fn get_textbook_versions(&self) -> Result<Vec<TextbookVersion>>;

    /// Create a new textbook version
    async fn create_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion>;

    /// Update an existing textbook version
    async fn update_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion>;

    /// Delete a textbook version
    async fn delete_textbook_version(&self, id: i32) -> Result<()>;
}
