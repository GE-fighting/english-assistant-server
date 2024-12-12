use crate::models::textbook::Textbook;
use serde::{Deserialize, Serialize};
use time::format_description;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextbookDTO {
    pub id: Option<i32>,
    pub version_id: Option<i32>,
    pub grade_id: Option<i32>,
    pub semester_id: Option<i32>,
    pub created_at: Option<String>,
    pub name: Option<String>,
    pub unit_count: Option<i32>,
    pub word_count: Option<i32>,
    pub textbook_version: Option<String>,
    pub grade: Option<String>,
    pub semester: Option<String>,
}

//根据textbook生成textdto
impl From<Textbook> for TextbookDTO {
    fn from(textbook: Textbook) -> Self {
        let format = format_description::well_known::Rfc3339;
        Self {
            id: textbook.id,
            version_id: textbook.version_id,
            grade_id: textbook.grade_id,
            semester_id: textbook.semester_id,
            created_at: Option::from(textbook.created_at.unwrap().format(&format).unwrap()),
            name: Some(textbook.name),
            unit_count: textbook.unit_count,
            word_count: textbook.word_count,
            textbook_version: textbook.textbook_version,
            grade: textbook.grade,
            semester: textbook.semester,
        }
    }
}
