use crate::domain::models::textbook_version::TextbookVersion;
use crate::domain::services::interfaces::textbook_version_service::TextbookVersionService;
use crate::infrastructure::database::repositories::TextbookVersionRepository;
use async_trait::async_trait;
use std::sync::Arc;

use anyhow::Result;

pub struct TextbookVersionServiceImpl {
    repository: Arc<dyn TextbookVersionRepository>,
}

impl TextbookVersionServiceImpl {
    pub fn new(repository: Arc<dyn TextbookVersionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl TextbookVersionService for TextbookVersionServiceImpl {
    async fn get_textbook_versions(&self) -> Result<Vec<TextbookVersion>> {
        self.repository.find_all().await
    }

    async fn create_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion> {
        self.repository.save(textbook_version).await
    }

    async fn update_textbook_version(
        &self,
        textbook_version: &TextbookVersion,
    ) -> Result<TextbookVersion> {
        self.repository.save(textbook_version).await
    }

    async fn delete_textbook_version(&self, id: i32) -> Result<()> {
        self.repository.delete(id).await
    }
}
