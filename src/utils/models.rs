use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct PathParamsModel {
    pub id: uuid::Uuid
}

#[derive(Serialize)]
pub struct HttpClientResponse {
    pub code: u16,
    pub status: bool,
    pub prompt: String,
    pub message: String,
    pub data: Value
}

pub struct SaveMediaDto {
    pub file_name: String,
    pub mime_type: String,
    pub file_path: String,
    pub file_size: i64,
    pub media_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum ResponseCode {
    Success = 2000,
    Failed = 2001,
}

impl ResponseCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn prompt(&self) -> &'static str {
        match self {
            ResponseCode::Success => "Success",
            ResponseCode::Failed => "Failed",
        }
    }

    pub fn status(&self) -> bool {
        matches!(self, ResponseCode::Success)
    }
}

impl HttpClientResponse {
    pub fn new(code: ResponseCode, message: String, data: Value) -> Self {
        Self {
            code: code.as_u16(),
            status: code.status(),
            prompt: code.prompt().to_string(),
            message,
            data,
        }
    }
}