use crate::models::response::ApiResponse;
use crate::models::word::Word;
use crate::services::external_service::llm_service::LLMService;
use sqlx::PgPool;

pub async fn create_word(pool: &PgPool, word: &str) -> ApiResponse<Word> {
    let word_in_db = get_word(pool, word).await;
    if word_in_db.code == 200 {
        return ApiResponse::error(
            409,
            format!("Word '{}' already exists in the database", word),
        );
    }
    let pronunciation_us = format!("http://dict.youdao.com/dictvoice?type=0&audio={}", word);
    let pronunciation_uk = format!("http://dict.youdao.com/dictvoice?type=1&audio={}", word);

    let llm_service = LLMService::new();
    let (us_phonetic, uk_phonetic) = match llm_service.get_phonetics(&word).await {
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
        uk_phonetic,
        pronunciation_us,
        pronunciation_uk
    )
    .fetch_one(pool)
    .await
    {
        Ok(created_word) => ApiResponse::success(created_word),
        Err(e) => ApiResponse::error(500, format!("Database error: {}", e)),
    }
}

pub async fn get_word(pool: &PgPool, word: &str) -> ApiResponse<Word> {
    match sqlx::query_as!(
        Word,
        r#"
        SELECT * FROM words WHERE word = $1
        "#,
        word
    )
    .fetch_one(pool)
    .await
    {
        Ok(word) => ApiResponse::success(word),
        Err(sqlx::Error::RowNotFound) => {
            ApiResponse::error(404, format!("Word '{}' not found", word))
        }
        Err(e) => ApiResponse::error(500, format!("Database error: {}", e)),
    }
}

//批量更新词库（美式音标、英式音标、发音链接）
pub async fn update_batch_words(pool: &PgPool) -> ApiResponse<()> {
    //先读取所有数据，再批量更新
    let mut words: Vec<Word> = match sqlx::query_as!(Word, "SELECT * FROM words")
        .fetch_all(pool)
        .await
    {
        Ok(words) => words,
        Err(e) => return ApiResponse::error(500, format!("Database error: {}", e)),
    };
    for word in &mut words {
        //判断单词是否有英式音标
        if word.phonetic_uk.is_none() {
            let llm_service = LLMService::new();
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
       .execute(pool)
       .await
       {
           Ok(_) => (),
           Err(e) => return ApiResponse::error(500, format!("Database error: {}", e)),
       }
    }
    ApiResponse::success(())
}
