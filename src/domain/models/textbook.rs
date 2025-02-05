use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Textbook {
    pub id: Option<i32>,
    pub version_id: Option<i32>,
    pub grade_id: Option<i32>,
    pub semester_id: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
    pub name: String,
    pub unit_count: Option<i32>,
    pub word_count: Option<i32>,
    pub textbook_version: Option<String>,
    pub grade: Option<String>,
    pub semester: Option<String>,
    pub updated_at: Option<OffsetDateTime>,
}
