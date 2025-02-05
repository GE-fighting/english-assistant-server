use crate::infrastructure::dto::WordInfo;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ThirdPartyService: Send + Sync {
    async fn fetch_word_info(&self, word: &str) -> Result<WordInfo>;
}
