use crate::infrastructure::llm::impl_deepseek::DeepSeekServiceImpl;
use crate::infrastructure::llm::provider::{LLMConfig, LLMProvider};
use crate::infrastructure::llm::{LLMService, YiServiceImpl};
use anyhow::Result;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::env;
use std::sync::Arc;

lazy_static! {
    static ref PROVIDER_CACHE: DashMap<String, Arc<dyn LLMService + Send + Sync>> = DashMap::new();
}

pub trait LLMServiceFactoryTrait: Send + Sync {
    fn create_from_name(&self, name: &str) -> Result<Arc<dyn LLMService + Send + Sync>>;
}

pub struct LLMServiceFactory;

impl LLMServiceFactory {
    pub fn create(config: &LLMConfig) -> Result<Arc<dyn LLMService + Send + Sync>> {
        let cache_key = Self::generate_cache_key(&config);

        if let Some(cached) = PROVIDER_CACHE.get(&cache_key) {
            return Ok(cached.clone());
        }

        let service: Arc<dyn LLMService + Send + Sync> = match config.provider {
            LLMProvider::Yi => {
                let mut service = YiServiceImpl::new();
                service.configure(&config)?;
                Arc::new(service)
            }
            LLMProvider::DeepSeek => {
                let mut service = DeepSeekServiceImpl::new()?;
                service.configure(&config)?;
                Arc::new(service)
            }
        };

        PROVIDER_CACHE.insert(cache_key, service.clone());
        Ok(service)
    }

    fn generate_cache_key(config: &LLMConfig) -> String {
        format!(
            "{}:{}:{}:{}",
            config.provider.to_string(),
            config.api_key.as_deref().unwrap_or("default"),
            config.base_url.as_deref().unwrap_or("default"),
            config.model_name.as_deref().unwrap_or("default")
        )
    }
}

impl LLMServiceFactoryTrait for LLMServiceFactory {
    fn create_from_name(&self, name: &str) -> Result<Arc<dyn LLMService + Send + Sync>> {
        let provider = name.parse::<LLMProvider>()?;
        let mut config = LLMConfig::new(provider);

        let api_key = env::var(format!("LLM_{}_API_KEY", name.to_uppercase()))?;
        let base_url = env::var(format!("LLM_{}_BASE_URL", name.to_uppercase()))?;
        let model = env::var(format!("LLM_{}_MODEL", name.to_uppercase()))?;
        let timeout = env::var(format!("LLM_{}_TIMEOUT", name.to_uppercase()))?;

        config
            .with_api_key(api_key)
            .with_base_url(base_url)
            .with_model(model)
            .with_timeout(timeout.parse()?);

        Self::create(&config)
    }
}
