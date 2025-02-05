use crate::domain::models::model_provider::ModelProvider;
use crate::infrastructure::database::repositories::Repository;
use async_trait::async_trait;
use sqlx::{postgres::PgQueryResult, PgPool};
use std::sync::Arc;
use time::OffsetDateTime;

#[async_trait]
pub trait ModelProviderRepository: Repository<ModelProvider, i32> + Send + Sync {}

pub struct ModelProviderRepositoryImpl {
    pool: Arc<PgPool>,
}

impl ModelProviderRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ModelProviderRepositoryImpl { pool }
    }
}

#[async_trait]
impl Repository<ModelProvider, i32> for ModelProviderRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<ModelProvider>> {
        let provider = sqlx::query_as::<_, ModelProvider>(
            "SELECT * FROM model_providers WHERE provider_id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(provider)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<ModelProvider>> {
        let providers = sqlx::query_as::<_, ModelProvider>(
            "SELECT * FROM model_providers ORDER BY provider_id",
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(providers)
    }

    async fn save(&self, entity: &ModelProvider) -> anyhow::Result<ModelProvider> {
        let now = OffsetDateTime::now_utc();

        let saved_provider = match entity.provider_id {
            // Update existing provider
            Some(id) => {
                sqlx::query_as::<_, ModelProvider>(
                    r#"
                    UPDATE model_providers SET 
                        provider_name = $1,
                        api_base_url = $2,
                        api_key_required = $3,
                        model_types = $4,
                        is_active = $5,
                        updated_at = $6
                    WHERE provider_id = $7
                    RETURNING *
                    "#,
                )
                .bind(&entity.provider_name)
                .bind(&entity.api_base_url)
                .bind(&entity.api_key_required)
                .bind(&entity.model_types)
                .bind(&entity.is_active)
                .bind(now)
                .bind(id)
                .fetch_one(&*self.pool)
                .await?
            }
            // Insert new provider
            None => {
                sqlx::query_as::<_, ModelProvider>(
                    r#"
                    INSERT INTO model_providers (
                        provider_name, api_base_url, api_key_required, 
                        model_types, is_active, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING *
                    "#,
                )
                .bind(&entity.provider_name)
                .bind(&entity.api_base_url)
                .bind(&entity.api_key_required)
                .bind(&entity.model_types)
                .bind(&entity.is_active)
                .bind(now)
                .bind(now)
                .fetch_one(&*self.pool)
                .await?
            }
        };

        Ok(saved_provider)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let result: PgQueryResult =
            sqlx::query("DELETE FROM model_providers WHERE provider_id = $1")
                .bind(id)
                .execute(&*self.pool)
                .await?;

        if result.rows_affected() == 0 {
            anyhow::bail!("No model provider found with id: {}", id);
        }

        Ok(())
    }
}

#[async_trait]
impl ModelProviderRepository for ModelProviderRepositoryImpl {}
