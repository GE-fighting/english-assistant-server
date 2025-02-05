use super::Repository;
use crate::api::dto::unit_word_dto::WordPageRequestDTO;
use crate::domain::models::word::Word;
use crate::infrastructure::database::repositories::base::Paginated;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;

#[async_trait]
pub trait WordRepository: Repository<Word, i32> + Send + Sync {
    /// 根据单词查询
    async fn find_by_word(&self, word: &str) -> Result<Option<Word>>;

    /// 根据单元ID查询单词列表
    async fn find_by_unit_id(&self, unit_id: i32) -> Result<Vec<Word>>;

    /// 模糊搜索单词
    async fn search_words(&self, keyword: &str) -> Result<Vec<Word>>;

    /// 查询单词数量
    async fn count(&self) -> Result<u32>;

}
pub struct WordRepositoryImpl {
    pool: Arc<PgPool>,
}

impl WordRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Word, i32> for WordRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Word>> {
        let word = sqlx::query_as!(
            Word,
            r#"
            SELECT 
                word_id, word, phonetic_us, pronunciation_us, 
                phonetic_uk, pronunciation_uk, meaning, example,
                created_at, updated_at
            FROM words 
            WHERE word_id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(word)
    }

    async fn find_all(&self) -> Result<Vec<Word>> {
        let words = sqlx::query_as!(
            Word,
            r#"
            SELECT 
                word_id, word, phonetic_us, pronunciation_us, 
                phonetic_uk, pronunciation_uk, meaning, example,
                created_at, updated_at
            FROM words 
            ORDER BY word_id
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(words)
    }

    async fn save(&self, entity: &Word) -> Result<Word> {
        let result = if let Some(id) = entity.word_id {
            // Update
            sqlx::query_as!(
                Word,
                r#"
                UPDATE words 
                SET word = $1, 
                    phonetic_us = $2, 
                    pronunciation_us = $3,
                    phonetic_uk = $4,
                    pronunciation_uk = $5,
                    meaning = $6,
                    example = $7
                WHERE word_id = $8
                RETURNING word_id, word, phonetic_us, pronunciation_us, 
                          phonetic_uk, pronunciation_uk, meaning, example,
                          created_at, updated_at
                "#,
                entity.word,
                entity.phonetic_us,
                entity.pronunciation_us,
                entity.phonetic_uk,
                entity.pronunciation_uk,
                entity.meaning,
                entity.example,
                id
            )
            .fetch_one(&*self.pool)
            .await?
        } else {
            // Insert
            sqlx::query_as!(
                Word,
                r#"
                INSERT INTO words (
                    word, phonetic_us, pronunciation_us, 
                    phonetic_uk, pronunciation_uk, meaning, example
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING word_id, word, phonetic_us, pronunciation_us, 
                          phonetic_uk, pronunciation_uk, meaning, example,
                          created_at, updated_at
                "#,
                entity.word,
                entity.phonetic_us,
                entity.pronunciation_us,
                entity.phonetic_uk,
                entity.pronunciation_uk,
                entity.meaning,
                entity.example
            )
            .fetch_one(&*self.pool)
            .await?
        };

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM words 
            WHERE word_id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl WordRepository for WordRepositoryImpl {
    async fn find_by_word(&self, word: &str) -> Result<Option<Word>> {
        let word = sqlx::query_as!(
            Word,
            r#"
            SELECT 
                word_id, word, phonetic_us, pronunciation_us, 
                phonetic_uk, pronunciation_uk, meaning, example,
                created_at, updated_at
            FROM words 
            WHERE word = $1
            "#,
            word
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(word)
    }

    async fn find_by_unit_id(&self, unit_id: i32) -> Result<Vec<Word>> {
        let words = sqlx::query_as!(
            Word,
            r#"
            SELECT 
                w.word_id, w.word, w.phonetic_us, w.pronunciation_us, 
                w.phonetic_uk, w.pronunciation_uk, w.meaning, w.example,
                w.created_at, w.updated_at
            FROM words w
            JOIN word_unit_mappings wum ON w.word_id = wum.word_id
            WHERE wum.unit_id = $1
            ORDER BY wum.id
            "#,
            unit_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(words)
    }

    async fn search_words(&self, keyword: &str) -> Result<Vec<Word>> {
        let search_pattern = format!("%{}%", keyword);
        let words = sqlx::query_as!(
            Word,
            r#"
            SELECT 
                word_id, word, phonetic_us, pronunciation_us, 
                phonetic_uk, pronunciation_uk, meaning, example,
                created_at, updated_at
            FROM words 
            WHERE word ILIKE $1 OR meaning ILIKE $1
            ORDER BY word
            "#,
            search_pattern
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(words)
    }

    async fn count(&self) -> Result<u32> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM words
            "#
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| anyhow!(e))?;
        Ok(count.unwrap() as u32)
    }
}
