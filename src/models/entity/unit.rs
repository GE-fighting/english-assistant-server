use crate::models::dto::unit_dto::UnitDTO;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Unit {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub textbook_id: Option<i32>,
    pub sequence_number: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
    pub word_count: Option<i32>,
    pub updated_at: Option<OffsetDateTime>,
}

//实现构造函数
impl Unit {
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
