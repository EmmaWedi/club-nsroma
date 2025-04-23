use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::libs::validator::{validate_mime_type, validate_id_type, validate_base64_file_size};

#[derive(Debug, Validate, Deserialize)]
pub struct PathParamsModel {
    pub id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct HttpClientResponse {
    pub code: u16,
    pub status: bool,
    pub prompt: String,
    pub message: String,
    pub data: Value,
}

pub struct SaveMediaDto {
    pub file_name: String,
    pub mime_type: String,
    pub file_path: String,
    pub file_size: i64,
    pub media_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
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

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ImageUploadParams {
    #[validate(custom(function = "validate_base64_file_size"))]
    pub img: String,
    #[validate(custom(function = "validate_mime_type"))]
    pub mime_type: String,
    #[validate(range(min = 0, message = "File size must be a non-negative number"))]
    pub file_size: i64,
    #[validate(length(min = 1, message = "Media type must not be empty"))]
    pub media_type: String,
    #[validate(range(min = 1, message = "Width must be a positive number"))]
    pub width: Option<i32>,
    #[validate(range(min = 1, message = "Height must be a positive number"))]
    pub height: Option<i32>,
    #[validate(custom(function = "validate_id_type"))]
    pub id_type: String,
    #[validate(length(min = 1, message = "id number must not be empty"))]
    pub id_nun: String,
}


pub struct SaveMediaFilesDto {
    pub id: uuid::Uuid,
    pub dir: String,
    pub data: String,
    pub mime_type: String,
    pub media_type: String,
    pub size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>
}