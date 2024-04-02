use rocket::{
    http::Status,
    response::{status::Custom, Responder},
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub code: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
}

impl APIResponse<()> {
    pub fn new_success<T>(data: impl Into<Option<T>>, message: &str) -> APIResponse<T> {
        APIResponse {
            code: Status::Ok,
            data: data.into(),
            message: message.to_string(),
        }
    }

    /// Create a new success response with no message
    pub fn new_success_nm<T>(data: impl Into<Option<T>>) -> APIResponse<T> {
        APIResponse {
            code: Status::Ok,
            data: data.into(),
            message: "".to_string(),
        }
    }

    pub fn new_error<T>(code: Status, message: &str) -> APIResponse<T> {
        APIResponse {
            code,
            data: None,
            message: message.to_string(),
        }
    }

    pub fn new_internal_error<T>(message: &str) -> APIResponse<T> {
        APIResponse {
            code: Status::InternalServerError,
            data: None,
            message: message.to_string(),
        }
    }

    pub fn new<T>(code: Status, data: impl Into<Option<T>>, message: String) -> APIResponse<T> {
        APIResponse {
            code,
            data: data.into(),
            message,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum APIError {
    InternalError(String),
    NotFound(String),
    BadRequest(String),
}

impl<'r> Responder<'r, 'static> for APIError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let error = ErrorResponse {
            status: match self {
                APIError::InternalError(_) => Status::InternalServerError,
                APIError::NotFound(_) => Status::NotFound,
                APIError::BadRequest(_) => Status::BadRequest,
            },
            message: match self {
                APIError::InternalError(msg)
                | APIError::NotFound(msg)
                | APIError::BadRequest(msg) => msg,
            },
        };

        return Custom(error.status, Json(error)).respond_to(request);
    }
}

impl From<mongodb::error::Error> for APIError {
    fn from(err: mongodb::error::Error) -> Self {
        APIError::InternalError(err.to_string())
    }
}

pub type ApiResult<T> = Result<Json<APIResponse<T>>, APIError>;

pub fn ok<T>(data: T) -> ApiResult<T> {
    Ok(Json(APIResponse::new_success_nm(data)))
}
