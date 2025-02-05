use std::fmt::{Debug, Formatter};
use crate::config::CacheKeys;
use crate::domain::services::interfaces::SystemConfigService;
use crate::infrastructure::cache::redis::RedisOperations;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

pub struct SystemConfigServiceImpl {
    cache: Arc<dyn RedisOperations>,
}

impl SystemConfigServiceImpl {
    pub fn new(cache: Arc<dyn RedisOperations>) -> Self {
        Self { cache }
    }
}


#[async_trait]
impl SystemConfigService for SystemConfigServiceImpl {
    async fn set_use_model(&self, model_name: &str) -> Result<()> {
        let key = CacheKeys::get_key(CacheKeys::MODEL_PREFIX, "use");
        self.cache.set_key(key.as_str(), model_name, None).await?;
        Ok(())
    }

    async fn get_use_model(&self) -> Result<String> {
        let key = CacheKeys::get_key(CacheKeys::MODEL_PREFIX, "use");
        let model = self.cache.get_key(key.as_str()).await?;
        if model.is_none() {
            return Err(anyhow::Error::msg("No model found in cache"));
        }
        Ok(model.unwrap())
    }
}
