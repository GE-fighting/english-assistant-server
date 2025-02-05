use async_trait::async_trait;
use crate::domain::models::model_provider::ModelProvider;

#[async_trait]
pub trait ModelProviderService: Send + Sync {
    /// Get all model providers
    async fn get_all_providers(&self) -> anyhow::Result<Vec<ModelProvider>>;
}
