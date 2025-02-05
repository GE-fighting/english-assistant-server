use anyhow::Result;
use async_trait::async_trait;

use crate::api::dto::unit_dto::UnitDTO;

#[async_trait]
pub trait UnitService: Send + Sync {
    async fn create_unit(&self, unit: &UnitDTO) -> Result<UnitDTO>;
    async fn get_units(&self, unit_dto: &UnitDTO) -> Result<Vec<UnitDTO>>;
    async fn delete_unit(&self, id: i32) -> Result<()>;
}
