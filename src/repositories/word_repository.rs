use crate::models::word::Word;
use sqlx::PgPool;

// 根据单词查询记录
pub async fn find_by_word(pool: &PgPool, word: &str) -> Result<Option<Word>, sqlx::Error> {
    let record = sqlx::query_as!(Word, "SELECT * FROM words WHERE word = $1", word)
        .fetch_optional(pool)
        .await?;
    Ok(record)
}

// 根据id查询记录
pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Word>, sqlx::Error> {
    let record = sqlx::query_as!(Word, "SELECT * FROM words WHERE word_id = $1", id)
        .fetch_optional(pool)
        .await?;
    Ok(record)
}

// 创建记录
pub async fn create(pool: &PgPool, word_entity: &Word) -> Result<Word, sqlx::Error> {
    // 检查必填字段是否为空
    if word_entity.word.is_empty() {
        return Err(sqlx::Error::Protocol("Word cannot be empty".to_string()));
    }
    let record = sqlx::query_as!(Word,
            "INSERT INTO words (word, phonetic_us, phonetic_uk, pronunciation_us, pronunciation_uk, meaning, example) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            word_entity.word,
            word_entity.phonetic_us,
            word_entity.phonetic_uk,
            word_entity.pronunciation_us,
            word_entity.pronunciation_uk,
            word_entity.meaning,
            word_entity.example
        )
        .fetch_one(pool)
        .await?;

    Ok(record)
}

pub async fn update(pool: &PgPool, word_entity: &Word) -> Result<Word, sqlx::Error> {
    let record = sqlx::query_as!(Word,
            "UPDATE words SET word = $1, phonetic_us = $2, phonetic_uk = $3, pronunciation_us = $4, pronunciation_uk = $5, meaning = $6, example = $7
             WHERE word_id = $8 RETURNING *",
            word_entity.word,
            word_entity.phonetic_us,
            word_entity.phonetic_uk,
            word_entity.pronunciation_us,
            word_entity.pronunciation_uk,
            word_entity.meaning,
            word_entity.example,
            word_entity.word_id
        )
        .fetch_one(pool)
        .await?;
    Ok(record)
}
