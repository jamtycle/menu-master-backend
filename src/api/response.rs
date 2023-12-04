use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub code: u16,
    pub data: T,
    pub message: String,
}