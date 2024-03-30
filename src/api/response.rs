use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub code: u16,
    pub data: Option<T>,
    pub message: String,
}

impl APIResponse<()> {
    pub fn new_success<T>(data: impl Into<Option<T>>, message: &str) -> APIResponse<T> {
        APIResponse {
            code: 200,
            data: data.into(),
            message: message.to_string(),
        }
    }

    /// Create a new success response with no message
    pub fn new_success_nm<T>(data: impl Into<Option<T>>) -> APIResponse<T> {
        APIResponse {
            code: 200,
            data: data.into(),
            message: "".to_string(),
        }
    }

    pub fn new_error<T>(message: &str) -> APIResponse<T> {
        APIResponse {
            code: 500,
            data: None,
            message: message.to_string(),
        }
    }

    pub fn new<T>(code: u16, data: impl Into<Option<T>>, message: String) -> APIResponse<T> {
        APIResponse {
            code,
            data: data.into(),
            message,
        }
    }
}
