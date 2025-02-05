use super::base::Repository;
use crate::api::dto::unit_word_dto::WordDTO;
use crate::domain::models::word::Word;
use crate::domain::models::word_unit_mapping::WordUnitMapping;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait WordUnitMappingRepository: Repository<WordUnitMapping, i32> + Send + Sync {
    async fn find_word_by_unit_id(&self, unit_id: i32) -> Result<Vec<Word>>;

    async fn find_word_dto_by_unit_id(&self, unit_id: i32) -> Result<Vec<WordDTO>>;

    /// 根据单词ID查询映射关系
    async fn find_by_word_id(&self, word_id: i32) -> Result<Vec<WordUnitMapping>>;

    /// 批量保存映射关系
    async fn batch_save(&self, mappings: &[WordUnitMapping]) -> Result<Vec<WordUnitMapping>>;
}

pub struct WordUnitMappingRepositoryImpl {
    pool: Arc<PgPool>,
}

impl WordUnitMappingRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<WordUnitMapping, i32> for WordUnitMappingRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<WordUnitMapping>> {
        let mapping = sqlx::query_as!(
            WordUnitMapping,
            "SELECT * FROM word_unit_mappings WHERE id = $1",
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(mapping)
    }

    async fn find_all(&self) -> Result<Vec<WordUnitMapping>> {
        let mappings = sqlx::query_as!(
            WordUnitMapping,
            "SELECT * FROM word_unit_mappings ORDER BY id"
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(mappings)
    }

    async fn save(&self, mapping: &WordUnitMapping) -> Result<WordUnitMapping> {
        let result = if let Some(id) = mapping.id {
            // Update
            sqlx::query_as!(
                WordUnitMapping,
                r#"
                UPDATE word_unit_mappings
                SET word_id = $1, unit_id = $2
                WHERE id = $3
                RETURNING *
                "#,
                mapping.word_id,
                mapping.unit_id,
                id
            )
            .fetch_one(&*self.pool)
            .await?
        } else {
            // Insert
            sqlx::query_as!(
                WordUnitMapping,
                r#"
                INSERT INTO word_unit_mappings (word_id, unit_id)
                VALUES ($1, $2)
                RETURNING *
                "#,
                mapping.word_id,
                mapping.unit_id,
            )
            .fetch_one(&*self.pool)
            .await?
        };

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM word_unit_mappings WHERE id = $1", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl WordUnitMappingRepository for WordUnitMappingRepositoryImpl {
    async fn find_word_by_unit_id(&self, unit_id: i32) -> Result<Vec<Word>> {
        let words = sqlx::query_as!(
            Word,
            r#"
            select w.*
            from word_unit_mappings wum
            right join words w
            on wum.word_id = w.word_id
            where wum.unit_id = $1
            order by w.word
            "#,
            unit_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(words)
    }

    async fn find_word_dto_by_unit_id(&self, unit_id: i32) -> Result<Vec<WordDTO>> {
        let word_dto = sqlx::query_as!(
            WordDTO,
            r#"
            select
            wum.id,
            wum.word_id,
            w.word,
            w.meaning,
            w.example,
            TO_CHAR(wum.created_at, 'YYYY-MM-DD HH:MI:SS') as created_at,
            TO_CHAR(wum.updated_at, 'YYYY-MM-DD HH:MI:SS') as updated_at,
            wum.unit_id,
            w.phonetic_us,
            w.phonetic_uk,
            w.pronunciation_us,
            w.pronunciation_uk
            from word_unit_mappings wum
            right join words w
            on wum.word_id = w.word_id
            where wum.unit_id = $1
            order by w.word
            "#,
            unit_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(word_dto)
    }

    async fn find_by_word_id(&self, word_id: i32) -> Result<Vec<WordUnitMapping>> {
        let mappings = sqlx::query_as!(
            WordUnitMapping,
            r#"
            SELECT * FROM word_unit_mappings
            WHERE word_id = $1 
            ORDER BY id
            "#,
            word_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(mappings)
    }

    async fn batch_save(&self, mappings: &[WordUnitMapping]) -> Result<Vec<WordUnitMapping>> {
        let mut tx = self.pool.begin().await?;
        let mut results = Vec::with_capacity(mappings.len());

        for mapping in mappings {
            let result = if let Some(id) = mapping.id {
                sqlx::query_as!(
                    WordUnitMapping,
                    r#"
                    UPDATE word_unit_mappings
                    SET word_id = $1, unit_id = $2
                    WHERE id = $3
                    RETURNING *
                    "#,
                    mapping.word_id,
                    mapping.unit_id,
                    id
                )
                .fetch_one(&mut *tx)
                .await?
            } else {
                sqlx::query_as!(
                    WordUnitMapping,
                    r#"
                    INSERT INTO word_unit_mappings (word_id, unit_id)
                    VALUES ($1, $2)
                    RETURNING *
                    "#,
                    mapping.word_id,
                    mapping.unit_id
                )
                .fetch_one(&mut *tx)
                .await?
            };
            results.push(result);
        }

        tx.commit().await?;
        Ok(results)
    }
}
