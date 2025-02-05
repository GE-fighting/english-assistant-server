use std::fmt::Debug;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SystemConfigService: Send + Sync {
    // 设置llm模型
    async fn set_use_model(&self, model_name: &str) -> Result<()>;

    async fn get_use_model(&self) -> Result<String>;
}
