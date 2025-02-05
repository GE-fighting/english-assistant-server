use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use super::base::Repository;
use crate::domain::models::textbook_version::TextbookVersion;

#[async_trait]
pub trait TextbookVersionRepository: Repository<TextbookVersion, i32> + Sync + Send {
    /// 根据名称查找教材版本
    async fn find_by_name(&self, name: &str) -> Result<Option<TextbookVersion>>;
}

pub struct TextbookVersionRepositoryImpl {
    pool: Arc<PgPool>,
}

impl TextbookVersionRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<TextbookVersion, i32> for TextbookVersionRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<TextbookVersion>> {
        let version = sqlx::query_as!(
            TextbookVersion,
            r#"
            SELECT * FROM textbook_versions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(version)
    }

    async fn find_all(&self) -> Result<Vec<TextbookVersion>> {
        let versions = sqlx::query_as!(
            TextbookVersion,
            r#"
            SELECT * FROM textbook_versions
            ORDER BY id
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(versions)
    }

    async fn save(&self, version: &TextbookVersion) -> Result<TextbookVersion> {
        let result = if let Some(id) = version.id {
            // Update
            sqlx::query_as!(
                TextbookVersion,
                r#"
                UPDATE textbook_versions
                SET name = $1
                WHERE id = $2
                RETURNING *
                "#,
                version.name,
                id
            )
            .fetch_one(&*self.pool)
            .await?
        } else {
            // Insert
            sqlx::query_as!(
                TextbookVersion,
                r#"
                INSERT INTO textbook_versions (name)
                VALUES ($1)
                RETURNING *
                "#,
                version.name
            )
            .fetch_one(&*self.pool)
            .await?
        };

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM textbook_versions
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl TextbookVersionRepository for TextbookVersionRepositoryImpl {
    async fn find_by_name(&self, name: &str) -> Result<Option<TextbookVersion>> {
        let version = sqlx::query_as!(
            TextbookVersion,
            r#"
            SELECT * FROM textbook_versions
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(version)
    }
}
