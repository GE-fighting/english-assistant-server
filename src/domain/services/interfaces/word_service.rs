use async_trait::async_trait;

use crate::domain::models::word::Word;

use anyhow::Result;

#[async_trait]
pub trait WordService: Send + Sync {
    async fn create_word(&self, word: &str) -> Result<Word>;
    async fn get_word(&self, word: &str) -> Result<Word>;
    async fn update_batch_words(&self) -> Result<()>;
}
