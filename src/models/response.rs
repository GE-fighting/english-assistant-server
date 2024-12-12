use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn not_found(message: String) -> Self {
        Self {
            code: 404,
            message,
            data: None,
        }
    }

    pub fn error_default_code(message: String) -> Self {
        Self {
            code: 500,
            message,
            data: None,
        }
    }
}
