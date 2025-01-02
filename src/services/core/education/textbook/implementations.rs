use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;
use crate::models::textbook::Textbook;
use crate::repositories::{textbook_repository, unit_repository};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use super::interface::TextbookService;

pub struct TextbookServiceImpl {
    pool: PgPool,
}

impl TextbookServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TextbookService for TextbookServiceImpl {
    async fn create_textbook(&self, textbook: &Textbook) -> ApiResponse<Textbook> {
        let textbook_version_name = sqlx::query_scalar!(
            "SELECT name FROM textbook_versions WHERE id = $1",
            textbook.version_id
        )
        .fetch_one(&self.pool)
        .await;

        let grade_name =
            sqlx::query_scalar!("SELECT name FROM grades WHERE id = $1", textbook.grade_id)
                .fetch_one(&self.pool)
                .await;

        let semester_name = sqlx::query_scalar!(
            "SELECT name FROM semesters WHERE id = $1",
            textbook.semester_id
        )
        .fetch_one(&self.pool)
        .await;

        match sqlx::query_as!(
            Textbook,
            "INSERT INTO textbooks (version_id, grade_id, semester_id, name, textbook_version, grade, semester)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            textbook.version_id,
            textbook.grade_id,
            textbook.semester_id,
            textbook.name,
            textbook_version_name.unwrap(),
            grade_name.unwrap(),
            semester_name.unwrap()
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(textbook) => ApiResponse::success(textbook),
            Err(err) => ApiResponse::error(500, format!("Error creating textbook: {}", err)),
        }
    }

    async fn get_textbooks(&self) -> ApiResponse<Vec<TextbookDTO>> {
        match sqlx::query_as!(Textbook, "SELECT * FROM textbooks")
            .fetch_all(&self.pool)
            .await
        {
            Ok(textbooks) => {
                let result: Vec<TextbookDTO> =
                    textbooks.into_iter().map(TextbookDTO::from).collect();
                ApiResponse::success(result)
            }
            Err(err) => ApiResponse::error(500, format!("Error fetching textbooks: {}", err)),
        }
    }

    async fn delete_textbook(&self, textbook_dto: &TextbookDTO) -> Result<()> {
        textbook_repository::delete_textbook(&self.pool, textbook_dto.id.unwrap()).await
    }

    async fn get_unit_by_textbook(&self, textbook_dto: &TextbookDTO) -> Result<Vec<UnitDTO>> {
        unit_repository::get_unit_by_textbook_dto(&self.pool, textbook_dto)
            .await
            .map(|units| units.into_iter().map(UnitDTO::from).collect())
    }
}
