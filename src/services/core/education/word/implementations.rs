use super::interface::WordService;
use crate::models::response::ApiResponse;
use crate::models::word::Word;
use crate::services::external::llm::{LLMService, LLMServiceImpl};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct WordServiceImpl {
    pool: PgPool,
}

impl WordServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WordService for WordServiceImpl {
    async fn create_word(&self, word: &str) -> ApiResponse<Word> {
        let word_in_db = self.get_word(word).await;
        if word_in_db.code == 200 {
            return ApiResponse::error(
                409,
                format!("Word '{}' already exists in the database", word),
            );
        }

        let pronunciation_us = format!("http://dict.youdao.com/dictvoice?type=0&audio={}", word);
        let pronunciation_uk = format!("http://dict.youdao.com/dictvoice?type=1&audio={}", word);

        let llm_service = LLMServiceImpl::new();
        let (us_phonetic, uk_phonetic) = match llm_service.get_phonetics(word).await {
            Ok(phonetics) => phonetics,
            Err(e) => return ApiResponse::error(500, format!("Error getting phonetics: {}", e)),
        };

        match sqlx::query_as!(
            Word,
            r#"
            INSERT INTO words (word, phonetic_us, pronunciation_us, phonetic_uk, pronunciation_uk)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            word,
            us_phonetic,
            pronunciation_us,
            uk_phonetic,
            pronunciation_uk
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(created_word) => ApiResponse::success(created_word),
            Err(e) => ApiResponse::error(500, format!("Database error: {}", e)),
        }
    }

    async fn get_word(&self, word: &str) -> ApiResponse<Word> {
        match sqlx::query_as!(
            Word,
            r#"
            SELECT * FROM words WHERE word = $1
            "#,
            word
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(word) => ApiResponse::success(word),
            Err(sqlx::Error::RowNotFound) => {
                ApiResponse::error(404, format!("Word '{}' not found", word))
            }
            Err(e) => ApiResponse::error(500, format!("Database error: {}", e)),
        }
    }

    async fn update_batch_words(&self) -> ApiResponse<()> {
        let mut words: Vec<Word> = match sqlx::query_as!(Word, "SELECT * FROM words")
            .fetch_all(&self.pool)
            .await
        {
            Ok(words) => words,
            Err(e) => return ApiResponse::error(500, format!("Database error: {}", e)),
        };

        for word in &mut words {
            if word.phonetic_uk.is_none() {
                let llm_service = LLMServiceImpl::new();
                let (us_phonetic, uk_phonetic) = match llm_service.get_phonetics(&word.word).await {
                    Ok(phonetics) => phonetics,
                    Err(e) => {
                        return ApiResponse::error(500, format!("Error getting phonetics: {}", e))
                    }
                };
                word.phonetic_us = Some(us_phonetic);
                word.phonetic_uk = Some(uk_phonetic);
            }

            word.pronunciation_us = Some(format!(
                "http://dict.youdao.com/dictvoice?type=0&audio={}",
                word.word
            ));
            word.pronunciation_uk = Some(format!(
                "http://dict.youdao.com/dictvoice?type=1&audio={}",
                word.word
            ));

            match sqlx::query!(
                "UPDATE words SET phonetic_us = $1, phonetic_uk = $2, pronunciation_us = $3, pronunciation_uk = $4 WHERE word = $5",
                word.phonetic_us,
                word.phonetic_uk,
                word.pronunciation_us,
                word.pronunciation_uk,
                word.word
            )
            .execute(&self.pool)
            .await
            {
                Ok(_) => (),
                Err(e) => return ApiResponse::error(500, format!("Database error: {}", e)),
            }
        }
        ApiResponse::success(())
    }
}
