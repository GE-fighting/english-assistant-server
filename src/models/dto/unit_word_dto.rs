use crate::models::entity::word_unit_mapping::WordUnitMapping;
use crate::models::word::Word;
use serde::{Deserialize, Serialize};
use time::format_description;

#[derive(Serialize, Deserialize, Clone)]
pub struct UnitWordDTO {
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

//实现UnitWordDTO的两个参数的构造函数
impl UnitWordDTO {
    pub fn new(word: &Word, unit_word: &WordUnitMapping) -> Self {
        let format = format_description::well_known::Rfc3339;
        Self {
            id: Some(unit_word.id),
            word_id: Some(unit_word.word_id),
            word: Some(word.word.clone()),
            meaning: word.meaning.clone(),
            example: word.example.clone(),
            created_at: Option::from(unit_word.created_at.unwrap().format(&format).unwrap()),
            updated_at: Option::from(unit_word.updated_at.unwrap().format(&format).unwrap()),
            phonetic_us: word.phonetic_us.clone(),
            phonetic_uk: word.phonetic_uk.clone(),
            pronunciation_us: word.pronunciation_us.clone(),
            pronunciation_uk: word.pronunciation_uk.clone(),
            unit_id: Some(unit_word.unit_id),
        }
    }
}
