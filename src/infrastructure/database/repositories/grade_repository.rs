use crate::domain::models::grade::Grade;
use crate::infrastructure::database::repositories::Repository;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use time::OffsetDateTime;

#[async_trait]
pub trait GradeRepository: Repository<Grade, i32> + Send + Sync {
    // 如果需要添加特定于Grade的方法，可以在这里添加
}

pub struct GradeRepositoryImpl {
    pool: Arc<PgPool>,
}

impl GradeRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Grade, i32> for GradeRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Grade>> {
        sqlx::query_as!(
            Grade,
            r#"
            SELECT id, name, created_at
            FROM grades
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| anyhow::Error::from(e))
    }

    async fn find_all(&self) -> Result<Vec<Grade>> {
        sqlx::query_as!(
            Grade,
            r#"
            SELECT id, name, created_at
            FROM grades
            ORDER BY id
            "#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| anyhow::Error::from(e))
    }

    async fn save(&self, entity: &Grade) -> Result<Grade> {
        let now = OffsetDateTime::now_utc();

        if let Some(id) = entity.id {
            // Update existing grade
            sqlx::query_as!(
                Grade,
                r#"
                UPDATE grades
                SET name = $1
                WHERE id = $2
                RETURNING id, name, created_at
                "#,
                entity.name,
                id
            )
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| anyhow::Error::from(e))
        } else {
            // Insert new grade
            sqlx::query_as!(
                Grade,
                r#"
                INSERT INTO grades (name, created_at)
                VALUES ($1, $2)
                RETURNING id, name, created_at
                "#,
                entity.name,
                now
            )
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| anyhow::Error::from(e))
        }
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM grades
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| anyhow::Error::from(e));
        Ok(())
    }
}

#[async_trait]
impl GradeRepository for GradeRepositoryImpl {
    // 实现特定于Grade的方法（如果有的话）
}
