use crate::models::word::Word;
use crate::models::ApiResponse;
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::fmt::format;

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

    match get_phonetics_from_ai(word).await {
        Ok((us_phonetic, uk_phonetic)) => {
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
        },
        Err(e) => ApiResponse::error(500, format!("Error getting phonetics: {}", e)),
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

async fn get_phonetics_from_ai(
    word: &str,
) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let api_key = "cff6431c84f34ce891ac6ba05af629ce";
    let api_url = "https://api.lingyiwanwu.com/v1/chat/completions";

    let prompt = format!(
        "Please provide the International Phonetic Alphabet (IPA) pronunciations for the English word '{}'. \
        Return the American (US) IPA and British (UK) IPA, \
        separated by a comma in that order. Do not include any additional text or explanation.",
        word
    );

    let response = client.post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(json!({
            "model": "yi-lightning",
            "messages": [
                {"role": "system", "content": "You are a linguistic expert specializing in English phonetics."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.3
        }).to_string())
        .send()
        .await?;

    let response_body: Value = serde_json::from_str(&response.text().await?)?;
    println!("response_body: {}", response_body);
    let content = response_body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract content from AI response")?
        .trim()
        .to_string();

    let mut content_iter = content.split(',');
    let us_phonetic = content_iter
        .next()
        .ok_or("Failed to extract US phonetic")?
        .trim()
        .to_string();
    let uk_phonetic = content_iter
        .next()
        .ok_or("Failed to extract UK phonetic")?
        .trim()
        .to_string();

    Ok((us_phonetic, uk_phonetic))
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
    for word in &mut words{
        //判断单词是否有英式音标
        if word.phonetic_uk.is_none() {
            match get_phonetics_from_ai(&word.word).await {
                Ok((us_phonetic, uk_phonetic)) => {
                    word.phonetic_us = Some(us_phonetic);
                    word.phonetic_uk = Some(uk_phonetic);
                }
                Err(e) => {
                    return ApiResponse::error(
                        500,
                        format!("Failed to get phonetics from AI: {}", e),
                    )
                }
            }
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
