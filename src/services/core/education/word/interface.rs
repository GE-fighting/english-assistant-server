use async_trait::async_trait;

use crate::models::response::ApiResponse;
use crate::models::word::Word;

#[async_trait]
pub trait WordService {
    async fn create_word(&self, word: &str) -> ApiResponse<Word>;
    async fn get_word(&self, word: &str) -> ApiResponse<Word>;
    async fn update_batch_words(&self) -> ApiResponse<()>;
}
