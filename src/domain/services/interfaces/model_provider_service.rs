use crate::domain::models::model_provider::ModelProvider;
use async_trait::async_trait;

#[async_trait]
pub trait ModelProviderService: Send + Sync {
    /// Get all model providers
    async fn get_all_providers(&self) -> anyhow::Result<Vec<ModelProvider>>;
}
