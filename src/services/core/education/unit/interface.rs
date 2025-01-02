use anyhow::Result;
use async_trait::async_trait;

use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;

#[async_trait]
pub trait UnitService {
    async fn create_unit(&self, unit: &UnitDTO) -> ApiResponse<UnitDTO>;
    async fn get_units(&self, unit_dto: &UnitDTO) -> ApiResponse<Vec<UnitDTO>>;
    async fn delete_unit(&self, unit_dto: &UnitDTO) -> Result<()>;
}
