use anyhow::Result;
use async_trait::async_trait;

use crate::api::dto::unit_word_dto::WordDTO;

#[async_trait]
pub trait WordUnitService: Send + Sync {
    // 得到单元所有单词
    async fn get_unit_words(&self, unit_id: i32) -> Result<Vec<WordDTO>>;
    // 绑定单词与单元
    async fn create_word_unit_mapping(&self, unit_word_dto: &WordDTO) -> Result<WordDTO>;
    // 删除单元中单词
    async fn delete_unit_word(&self, id: i32) -> Result<()>;
}
