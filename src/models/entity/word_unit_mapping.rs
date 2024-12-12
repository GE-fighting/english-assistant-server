use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct WordUnitMapping {
    pub id: i32,
    pub word_id: i32,
    pub unit_id: i32,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
