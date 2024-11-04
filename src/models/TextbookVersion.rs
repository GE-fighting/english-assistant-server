use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TextbookVersion {
    pub  id: Option<i32>,
    pub  name: String,
    pub created_at: Option<OffsetDateTime>
}

