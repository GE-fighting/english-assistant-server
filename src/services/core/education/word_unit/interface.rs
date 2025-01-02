use anyhow::Result;
use async_trait::async_trait;

use crate::models::dto::unit_word_dto::UnitWordDTO;
use crate::models::response::ApiResponse;

#[async_trait]
pub trait WordUnitService {
    async fn get_unit_words(&self, unit_id: i32) -> ApiResponse<Vec<UnitWordDTO>>;
    async fn create_word_unit_mapping(&self, unit_word_dto: &UnitWordDTO) -> Result<UnitWordDTO>;
    async fn delete_unit_word(&self, id: i32) -> Result<()>;
}
