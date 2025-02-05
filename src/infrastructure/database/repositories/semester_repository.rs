use crate::domain::models::semester::Semester;
use crate::infrastructure::database::repositories::Repository;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use time::OffsetDateTime;

#[async_trait]
pub trait SemesterRepository: Repository<Semester, i32> + Send + Sync {}

pub struct SemesterRepositoryImpl {
    pool: Arc<PgPool>,
}

impl SemesterRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Semester, i32> for SemesterRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<Semester>> {
        let semester = sqlx::query_as!(
            Semester,
            r#"
            SELECT id, name, created_at
            FROM semesters
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(semester)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Semester>> {
        let semesters = sqlx::query_as!(
            Semester,
            r#"
            SELECT id, name, created_at
            FROM semesters
            ORDER BY id
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(semesters)
    }

    async fn save(&self, entity: &Semester) -> anyhow::Result<Semester> {
        let now = OffsetDateTime::now_utc();

        if let Some(id) = entity.id {
            // Update existing semester
            let updated_semester = sqlx::query_as!(
                Semester,
                r#"
                UPDATE semesters
                SET name = $1
                WHERE id = $2
                RETURNING id, name, created_at
                "#,
                entity.name,
                id
            )
            .fetch_one(&*self.pool)
            .await?;

            Ok(updated_semester)
        } else {
            // Insert new semester
            let new_semester = sqlx::query_as!(
                Semester,
                r#"
                INSERT INTO semesters (name, created_at)
                VALUES ($1, $2)
                RETURNING id, name, created_at
                "#,
                entity.name,
                now
            )
            .fetch_one(&*self.pool)
            .await?;

            Ok(new_semester)
        }
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM semesters
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
impl SemesterRepository for SemesterRepositoryImpl {}
