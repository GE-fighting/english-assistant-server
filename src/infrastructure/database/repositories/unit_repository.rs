use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::sync::Arc;

use super::base::Repository;
use crate::api::dto::unit_dto::UnitDTO;
use crate::domain::models::unit::Unit;

#[async_trait]
pub trait UnitRepository: Repository<Unit, i32> + Send + Sync {
    /// 根据DTO条件查询单元列表
    async fn find_by_dto(&self, dto: &UnitDTO) -> Result<Vec<Unit>>;

    /// 根据教材ID查询单元列表
    async fn find_by_textbook_id(&self, textbook_id: Option<i32>) -> Result<Vec<Unit>>;
}

pub struct UnitRepositoryImpl {
    pool: Arc<PgPool>,
}

impl UnitRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn build_query_from_dto(&self, dto: &UnitDTO) -> QueryBuilder<'_, Postgres> {
        let mut query = QueryBuilder::new("SELECT * FROM units");
        let mut first_condition = true;

        if dto.id.is_some()
            || dto.name.is_some()
            || dto.textbook_id.is_some()
            || dto.sequence_number.is_some()
        {
            query.push(" WHERE ");

            if let Some(id) = dto.id {
                query.push("id = ").push_bind(id);
                first_condition = false;
            }

            if let Some(ref name) = dto.name {
                if !first_condition {
                    query.push(" AND ");
                }
                query.push("name LIKE ").push_bind(format!("%{}%", name));
                first_condition = false;
            }

            if let Some(textbook_id) = dto.textbook_id {
                if !first_condition {
                    query.push(" AND ");
                }
                query.push("textbook_id = ").push_bind(textbook_id);
                first_condition = false;
            }

            if let Some(sequence_number) = dto.sequence_number {
                if !first_condition {
                    query.push(" AND ");
                }
                query.push("sequence_number = ").push_bind(sequence_number);
            }
        }

        query.push(" ORDER BY sequence_number");
        query
    }
}

#[async_trait]
impl Repository<Unit, i32> for UnitRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Unit>> {
        let unit = sqlx::query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(unit)
    }

    async fn find_all(&self) -> Result<Vec<Unit>> {
        let units = sqlx::query_as!(Unit, "SELECT * FROM units ORDER BY id")
            .fetch_all(&*self.pool)
            .await?;

        Ok(units)
    }

    async fn save(&self, unit: &Unit) -> Result<Unit> {
        let result = if let Some(id) = unit.id {
            // Update
            sqlx::query_as!(
                Unit,
                r#"
                UPDATE units 
                SET name = $1, textbook_id = $2, sequence_number = $3, word_count = $4
                WHERE id = $5
                RETURNING *
                "#,
                unit.name,
                unit.textbook_id,
                unit.sequence_number,
                unit.word_count,
                id
            )
            .fetch_one(&*self.pool)
            .await?
        } else {
            // Insert
            sqlx::query_as!(
                Unit,
                r#"
                INSERT INTO units (name, textbook_id, sequence_number, word_count)
                VALUES ($1, $2, $3, $4)
                RETURNING *
                "#,
                unit.name,
                unit.textbook_id,
                unit.sequence_number,
                unit.word_count
            )
            .fetch_one(&*self.pool)
            .await?
        };

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM units WHERE id = $1", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl UnitRepository for UnitRepositoryImpl {
    async fn find_by_dto(&self, dto: &UnitDTO) -> Result<Vec<Unit>> {
        let mut query = self.build_query_from_dto(dto);
        let units = query
            .build_query_as::<Unit>()
            .fetch_all(&*self.pool)
            .await?;

        Ok(units)
    }

    async fn find_by_textbook_id(&self, textbook_id: Option<i32>) -> Result<Vec<Unit>> {
        let units = sqlx::query_as!(
            Unit,
            "SELECT * FROM units WHERE textbook_id = $1 ORDER BY sequence_number",
            textbook_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(units)
    }
}
