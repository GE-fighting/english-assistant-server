use crate::infrastructure::dto::WordInfo;
use crate::infrastructure::llm::provider::LLMConfig;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LLMService: Send + Sync {
    fn configure(&mut self, config: &LLMConfig) -> Result<()>;

    async fn get_phonetics(&self, word: &str) -> Result<(String, String)>;
    async fn get_example_sentences(&self, word: &str) -> Result<String>;
    async fn get_word_info(&self, word: &str) -> Result<WordInfo>;
}
