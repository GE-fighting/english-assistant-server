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
}

impl Word {
    pub fn new(word: String) -> Self {
        Self {
            word_id: None,
            word,
            phonetic_us: None,
            pronunciation_us: None,
            created_at: None,
            phonetic_uk: None,
            pronunciation_uk: None,
        }
    }
}
