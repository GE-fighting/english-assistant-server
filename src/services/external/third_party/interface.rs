use crate::services::external::third_party::implementations::WordInfo;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ThirdPartyService {
    async fn fetch_word_info(&self, word: &str) -> Result<WordInfo>;
}
