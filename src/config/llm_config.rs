use crate::infrastructure::llm::provider::LLMProvider;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LLMConfig {
    pub api_key: String,
    pub provider: String,
}

impl LLMConfig {
    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_provider(&self) -> Result<LLMProvider> {
        self.provider.parse()
    }
}
