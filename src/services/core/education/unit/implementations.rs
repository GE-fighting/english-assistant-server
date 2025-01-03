use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder};

use crate::models::dto::unit_dto::UnitDTO;
use crate::models::entity::unit::Unit;
use crate::models::response::ApiResponse;
use crate::repositories::unit_repository;

use super::interface::UnitService;

pub struct UnitServiceImpl {
    pool: PgPool,
}

impl UnitServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitService for UnitServiceImpl {
    async fn create_unit(&self, unit: &UnitDTO) -> ApiResponse<UnitDTO> {
        match sqlx::query_as!(
            Unit,
            "INSERT INTO units (name, textbook_id, sequence_number) VALUES ($1, $2, $3) RETURNING *",
            unit.name,
            unit.textbook_id,
            unit.sequence_number
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(unit) => ApiResponse::success(UnitDTO::from(unit)),
            Err(err) => ApiResponse::error(500, format!("Error creating unit: {}", err)),
        }
    }

    async fn get_units(&self, unit_dto: &UnitDTO) -> ApiResponse<Vec<UnitDTO>> {
        let mut query = QueryBuilder::new("SELECT * FROM units");

        if unit_dto.id.is_some()
            || unit_dto.name.is_some()
            || unit_dto.textbook_id.is_some()
            || unit_dto.sequence_number.is_some()
        {
            query.push(" WHERE ");

            let mut conditions = Vec::new();

            if let Some(id) = unit_dto.id {
                conditions.push(format!("id = {}", id));
            }

            if let Some(ref name) = unit_dto.name {
                conditions.push(format!("name = '{}'", name));
            }

            if let Some(textbook_id) = unit_dto.textbook_id {
                conditions.push(format!("textbook_id = {}", textbook_id));
            }

            if let Some(sequence_number) = unit_dto.sequence_number {
                conditions.push(format!("sequence_number = {}", sequence_number));
            }

            query.push(conditions.join(" AND "));
        }

        match query.build_query_as::<Unit>().fetch_all(&self.pool).await {
            Ok(units) => ApiResponse::success(units.into_iter().map(UnitDTO::from).collect()),
            Err(err) => ApiResponse::error(500, format!("Error fetching unit: {}", err)),
        }
    }

    async fn delete_unit(&self, unit_dto: &UnitDTO) -> Result<()> {
        unit_repository::delete_unit(&self.pool, unit_dto).await
    }
}
