use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::dto::textbook_dto::TextbookDTO;
use crate::domain::models::textbook::Textbook;

#[async_trait]
pub trait TextbookRepository: Send + Sync {
    /// 根据ID查找教材
    async fn find_by_id(&self, id: i32) -> Result<Option<Textbook>>;

    /// 根据DTO条件查询教材列表
    async fn find_by_dto(&self, dto: &TextbookDTO) -> Result<Vec<Textbook>>;

    /// 保存或更新教材
    async fn save(&self, textbook: &Textbook) -> Result<Textbook>;

    /// 根据ID删除教材
    async fn delete(&self, id: i32) -> Result<()>;

    /// 查询所有教材
    async fn find_all(&self) -> Result<Vec<Textbook>>;
}

pub struct TextbookRepositoryImpl {
    pool: Arc<PgPool>,
}

impl TextbookRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TextbookRepository for TextbookRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Textbook>> {
        sqlx::query_as::<_, Textbook>(
            r#"
            SELECT id, version_id, name, unit_count, word_count, textbook_version, grade, semester, grade_id, semester_id, created_at, updated_at
            FROM textbooks
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| anyhow::Error::from(e))
    }

    async fn find_by_dto(&self, dto: &TextbookDTO) -> Result<Vec<Textbook>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, version_id, name, unit_count, word_count, textbook_version, grade, semester, grade_id, semester_id, created_at, updated_at FROM textbooks WHERE 1=1",
        );

        if let Some(id) = dto.id {
            query.push(" AND id = ").push_bind(id);
        }
        if let Some(version_id) = dto.version_id {
            query.push(" AND version_id = ").push_bind(version_id);
        }
        if let Some(grade_id) = dto.grade_id {
            query.push(" AND grade_id = ").push_bind(grade_id);
        }
        if let Some(semester_id) = dto.semester_id {
            query.push(" AND semester_id = ").push_bind(semester_id);
        }

        query
            .build_query_as::<Textbook>()
            .fetch_all(&*self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

    async fn save(&self, textbook: &Textbook) -> Result<Textbook> {
        if let Some(id) = textbook.id {
            // Update existing textbook
            sqlx::query_as::<_, Textbook>(
                r#"
                UPDATE textbooks 
                SET version_id = $1, grade_id = $2, semester_id = $3,
                    name = $4, textbook_version = $5, grade = $6, semester = $7
                WHERE id = $8
                RETURNING *
                "#,
            )
            .bind(textbook.version_id)
            .bind(textbook.grade_id)
            .bind(textbook.semester_id)
            .bind(&textbook.name)
            .bind(&textbook.textbook_version)
            .bind(&textbook.grade)
            .bind(&textbook.semester)
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map_err(Into::<anyhow::Error>::into)
        } else {
            // Insert new textbook
            sqlx::query_as::<_, Textbook>(
                r#"
                INSERT INTO textbooks (version_id, grade_id, semester_id, name, 
                                     textbook_version, grade, semester)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
                "#,
            )
            .bind(textbook.version_id)
            .bind(textbook.grade_id)
            .bind(textbook.semester_id)
            .bind(&textbook.name)
            .bind(&textbook.textbook_version)
            .bind(&textbook.grade)
            .bind(&textbook.semester)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e: sqlx::Error| anyhow::Error::from(e))
        }
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM textbooks WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Textbook>> {
        sqlx::query_as::<_, Textbook>(
            r#"
            SELECT id, version_id, name, unit_count, word_count, textbook_version, grade, semester, grade_id, semester_id, created_at, updated_at
            FROM textbooks
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(anyhow::Error::from)
    }
}
