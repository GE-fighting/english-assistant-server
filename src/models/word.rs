use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Word {
    pub word_id: Option<i32>,
    pub word: String,
    pub phonetic_us: Option<String>,
    pub pronunciation_us: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub phonetic_uk: Option<String>,
    pub pronunciation_uk: Option<String>,
    pub updated_at: Option<PrimitiveDateTime>,
    pub meaning: Option<String>,
    pub example: Option<String>,
}

impl Word {
    pub fn new(word: &str) -> Self {
        Self {
            word_id: None,
            word: word.to_string(),
            phonetic_us: None,
            pronunciation_us: None,
            created_at: None,
            phonetic_uk: None,
            pronunciation_uk: None,
            updated_at: None,
            meaning: None,
            example: None,
        }
    }
}
