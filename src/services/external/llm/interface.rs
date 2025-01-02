use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LLMService {
    async fn get_phonetics(&self, word: &str) -> Result<(String, String)>;
    async fn get_example_sentences(&self, word: &str) -> Result<Vec<(String, String)>>;
}
