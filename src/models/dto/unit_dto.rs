use crate::models::entity::unit::Unit;
use serde::{Deserialize, Serialize};
use time::format_description;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UnitDTO {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub textbook_id: Option<i32>,
    pub sequence_number: Option<i32>,
    pub created_at: Option<String>,
    pub word_count: Option<i32>,
    pub updated_at: Option<String>,
}

impl UnitDTO {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            textbook_id: None,
            sequence_number: None,
            created_at: None,
            word_count: None,
            updated_at: None,
        }
    }
}

impl From<Unit> for UnitDTO {
    fn from(unit: Unit) -> Self {
        let format = format_description::well_known::Rfc3339;
        Self {
            id: unit.id,
            name: Option::from(unit.name),
            textbook_id: Option::from(unit.textbook_id),
            sequence_number: Option::from(unit.sequence_number),
            created_at: Option::from(unit.created_at.unwrap().format(&format).unwrap()),
            word_count: Option::from(unit.word_count),
            updated_at: Option::from(unit.updated_at.unwrap().format(&format).unwrap()),
        }
    }
}
