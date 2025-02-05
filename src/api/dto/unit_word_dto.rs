use crate::domain::models::word::Word;
use crate::domain::models::word_unit_mapping::WordUnitMapping;
use serde::{Deserialize, Serialize};
use time::format_description;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WordDTO {
    pub id: Option<i32>,
    pub word_id: Option<i32>,
    pub word: Option<String>,
    pub meaning: Option<String>,
    pub example: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub unit_id: Option<i32>,
    pub phonetic_us: Option<String>,
    pub phonetic_uk: Option<String>,
    pub pronunciation_us: Option<String>,
    pub pronunciation_uk: Option<String>,
}


impl WordDTO {
    pub fn new(word: &Word, unit_word: &WordUnitMapping) -> Self {
        let format = format_description::well_known::Rfc3339;

        Self {
            id: unit_word.id,
            word_id: unit_word.word_id,
            word: Some(word.word.clone()),
            meaning: word.meaning.clone(),
            example: word.example.clone(),
            created_at: Option::from(unit_word.created_at.unwrap().format(&format).unwrap()),
            updated_at: Option::from(unit_word.updated_at.unwrap().format(&format).unwrap()),
            phonetic_us: word.phonetic_us.clone(),
            phonetic_uk: word.phonetic_uk.clone(),
            pronunciation_us: word.pronunciation_us.clone(),
            pronunciation_uk: word.pronunciation_uk.clone(),
            unit_id: unit_word.unit_id,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WordPageRequestDTO {
    pub word_id: Option<i32>,
    pub word: Option<String>,
    pub meaning: Option<String>,
    pub example: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub unit_id: Option<i32>,
    pub phonetic_us: Option<String>,
    pub phonetic_uk: Option<String>,
    pub pronunciation_us: Option<String>,
    pub pronunciation_uk: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}



impl WordPageRequestDTO {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(10)
    }
}