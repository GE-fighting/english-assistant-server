use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WordInfo {
    pub us_phonetic: String,
    pub uk_phonetic: String,
    pub meanings: Vec<WordMeaning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordMeaning {
    pub pos: String,
    pub definition: String,
}
