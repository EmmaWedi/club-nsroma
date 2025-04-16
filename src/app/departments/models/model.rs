use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::validate_uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDepartmentDto {
    pub name: String,
    pub description: String,
    pub branch: uuid::Uuid,
    pub organization: uuid::Uuid,
    pub for_all: bool,
    pub employee_num: i32,
    pub leave_days: i32,
    pub daily_rate: Decimal,
    pub hourly_rate: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddDepartmentParams {
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 50, message = "Description is invalid"))]
    pub description: String,
    #[validate(custom(function = "validate_uuid"))]
    pub branch: String,
    #[validate(range(min = 0, max = 100, message = "Number of Employee is invalid"))]
    pub num_of_employee: i32,
    #[validate(range(min = 3, max = 20, message = "Number of Leave days is invalid"))]
    pub leave_days: i32,
    #[validate(range(min = 0.0, max = 100.0, message = "Daily rate must be non-negative"))]
    pub daily_rate: f64,
    #[validate(range(min = 0.0, max = 100.0, message = "Hourly rate must be non-negative"))]
    pub hourly_rate: f64,
}
