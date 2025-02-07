use crate::domain::models::model_provider::ModelProvider;
use crate::domain::services::interfaces::model_provider_service::ModelProviderService;
use crate::infrastructure::database::repositories::model_provider_repository::ModelProviderRepository;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ModelProviderServiceImpl {
    repository: Arc<dyn ModelProviderRepository>,
}

impl ModelProviderServiceImpl {
    pub fn new(repository: Arc<dyn ModelProviderRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ModelProviderService for ModelProviderServiceImpl {
    async fn get_all_providers(&self) -> anyhow::Result<Vec<ModelProvider>> {
        self.repository.find_all().await
    }
}
