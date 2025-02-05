use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LLMProvider {
    Yi,
    DeepSeek,
}

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub api_key: Option<String>,
    pub model_name: Option<String>,
    pub base_url: Option<String>,
    pub timeout: Option<u64>,
}

impl LLMConfig {
    pub fn new(provider: LLMProvider) -> Self {
        Self {
            provider,
            api_key: None,
            model_name: None,
            base_url: None,
            timeout: Some(30),
        }
    }

    pub fn with_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_model(&mut self, model_name: impl Into<String>) -> &mut Self {
        self.model_name = Some(model_name.into());
        self
    }

    pub fn with_base_url(&mut self, base_url: impl Into<String>) -> &mut Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
}

impl FromStr for LLMProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "yi" => Ok(LLMProvider::Yi),
            "deepseek" => Ok(LLMProvider::DeepSeek),
            _ => Err(anyhow!("Unsupported LLM provider: {}", s)),
        }
    }
}

impl ToString for LLMProvider {
    fn to_string(&self) -> String {
        match self {
            LLMProvider::Yi => "yi".to_string(),
            LLMProvider::DeepSeek => "deepseek".to_string(),
        }
    }
}
