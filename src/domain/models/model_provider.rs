use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ModelProvider {
    pub provider_id: Option<i32>,
    pub provider_name: Option<String>,
    pub api_base_url: Option<String>,
    pub api_key_required: Option<bool>,
    pub model_types: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl ModelProvider {
    pub fn new(
        provider_name: String,
        api_base_url: Option<String>,
        api_key_required: bool,
        model_types: Option<String>,
        is_active: bool,
    ) -> Self {
        Self {
            provider_id: None,
            provider_name: Some(provider_name),
            api_base_url,
            api_key_required: Some(api_key_required),
            model_types,
            is_active: Some(is_active),
            created_at: None,
            updated_at: None,
        }
    }
}
