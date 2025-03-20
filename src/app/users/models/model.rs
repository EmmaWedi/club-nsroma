use chrono::NaiveDate;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::validate_contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserDto {
    pub first_name: String,
    pub last_name: String,
    pub contact: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub organization: uuid::Uuid,
    pub session: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailsResponse {
    pub user: UserResponse,
    pub organization: entity::organizations::Model
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignInRequestModel {
    #[validate(custom(function = "validate_contact"))]
    pub phone: String,
    #[validate(length(min = 8, max = 14, message = "Password is invalid"))]
    pub password: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddUserParamsModel {
    #[validate(length(min = 3, max = 20, message = "First name is invalid"))]
    pub first_name: String,
    #[validate(length(min = 3, max = 20, message = "Last name is invalid"))]
    pub last_name: String,
    #[validate(custom(function = "validate_contact"))]
    pub contact: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub contact: String,
    pub email: String,
    pub is_blocked: bool,
    pub role_permission_id: Option<uuid::Uuid>,
    pub organization_id: uuid::Uuid,
    pub session: Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate
}

impl From<entity::users::Model> for UserResponse {
    fn from(user: entity::users::Model) -> Self {
        Self {
            first_name: user.first_name,
            last_name: user.last_name,
            contact: user.contact,
            email: user.email,
            is_blocked: user.is_blocked,
            role_permission_id: user.role_permissions_id,
            organization_id: user.organization_id,
            session: user.session,
            created_at: user.created_at.naive_utc().date(),
            updated_at: user.updated_at.naive_utc().date(),
        }
    }
}