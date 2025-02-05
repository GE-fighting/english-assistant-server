use crate::common::errors::ConversionError;
use crate::domain::models::textbook::Textbook;
use anyhow::Result;
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
    pub updated_at: Option<String>,
}

impl TextbookDTO {
    pub fn new() -> Self {
        Self {
            id: None,
            version_id: None,
            grade_id: None,
            semester_id: None,
            created_at: None,
            name: None,
            unit_count: None,
            word_count: None,
            textbook_version: None,
            grade: None,
            semester: None,
            updated_at: None,
        }
    }
}

impl TryFrom<Textbook> for TextbookDTO {
    type Error = ConversionError;

    fn try_from(textbook: Textbook) -> Result<Self, Self::Error> {
        let format = format_description::well_known::Rfc3339;

        let created_at = match textbook.created_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        let updated_at = match textbook.updated_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        Ok(Self {
            id: textbook.id,
            version_id: textbook.version_id,
            grade_id: textbook.grade_id,
            semester_id: textbook.semester_id,
            created_at,
            name: Some(textbook.name),
            unit_count: textbook.unit_count,
            word_count: textbook.word_count,
            textbook_version: textbook.textbook_version,
            grade: textbook.grade,
            semester: textbook.semester,
            updated_at,
        })
    }
}

impl TryFrom<TextbookDTO> for Textbook {
    type Error = ConversionError;

    fn try_from(dto: TextbookDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: dto.id,
            version_id: dto.version_id,
            grade_id: dto.grade_id,
            semester_id: dto.semester_id,
            created_at: None,
            name: dto.name.unwrap_or_default(),
            unit_count: dto.unit_count,
            word_count: dto.word_count,
            textbook_version: dto.textbook_version,
            grade: dto.grade,
            semester: dto.semester,
            updated_at: None,
        })
    }
}
