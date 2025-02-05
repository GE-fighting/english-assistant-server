use crate::api::dto::response::ApiResponse;
use anyhow::Result;

pub fn to_api_response<T>(result: Result<T>) -> ApiResponse<T> {
    match result {
        Ok(data) => ApiResponse::success(data),
        Err(error) => ApiResponse::error_default_code(error.to_string()),
    }
}
