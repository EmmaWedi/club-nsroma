use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::validate_contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganizationDto {
    pub name: String,
    pub post_code: String,
    pub location: String,
    pub country: uuid::Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddOrganizationParams {
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 20, message = "GPS is invalid"))]
    pub gps: String,
    #[validate(length(min = 3, max = 20, message = "Location is invalid"))]
    pub location: String,
    pub country: uuid::Uuid,
    #[validate(length(min = 3, max = 20, message = "First name is invalid"))]
    pub first_name: String,
    #[validate(length(min = 3, max = 20, message = "Last name is invalid"))]
    pub last_name: String,
    #[validate(custom(function = "validate_contact"))]
    pub contact: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnCredentialsModel {
    pub phone: String,
    pub password: String
}