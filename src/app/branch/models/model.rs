use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddBranchDto {
    pub name: String,
    pub organization: uuid::Uuid,
    pub location: String,
    pub gps: String,
    pub contact: String,
    pub email: Option<String>,
    pub country: uuid::Uuid
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddBranchParams {
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 20, message = "Location is invalid"))]
    pub location: String,
    #[validate(length(min = 3, max = 20, message = "GPS is invalid"))]
    pub gps: String,
    #[validate(length(equal = 10, message = "Contact is invalid"))]
    pub contact: String,
    #[validate(email)]
    pub email: Option<String>,
    pub organization: uuid::Uuid,
    pub country: uuid::Uuid
}