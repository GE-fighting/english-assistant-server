use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct WordUnitMapping {
    pub id: Option<i32>,
    pub word_id: Option<i32>,
    pub unit_id: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl WordUnitMapping {
    pub fn new() -> Self {
        WordUnitMapping {
            id: None,
            word_id: None,
            unit_id: None,
            created_at: None,
            updated_at: None,
        }
    }
}
