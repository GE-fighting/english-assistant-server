use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::api::dto::unit_dto::UnitDTO;
use crate::domain::models::unit::Unit;
use crate::domain::services::interfaces::unit_service::UnitService;
use crate::infrastructure::database::repositories::UnitRepository;

pub struct UnitServiceImpl {
    unit_repository: Arc<dyn UnitRepository>,
}

impl UnitServiceImpl {
    pub fn new(unit_repository: Arc<dyn UnitRepository>) -> Self {
        Self { unit_repository }
    }
}

#[async_trait]
impl UnitService for UnitServiceImpl {
    async fn create_unit(&self, unit_dto: &UnitDTO) -> Result<UnitDTO> {
        let unit = Unit::try_from(unit_dto)?;
        let saved_unit = self.unit_repository.save(&unit).await?;
        UnitDTO::try_from(&saved_unit).map_err(Into::into)
    }

    async fn get_units(&self, unit_dto: &UnitDTO) -> Result<Vec<UnitDTO>> {
        let units = self
            .unit_repository
            .find_by_textbook_id(unit_dto.textbook_id)
            .await?;

        units
            .into_iter()
            .map(|unit| UnitDTO::try_from(&unit).map_err(Into::into))
            .collect()
    }

    async fn delete_unit(&self, id: i32) -> Result<()> {
        self.unit_repository.delete(id).await
    }
}
