use chrono::{NaiveDate, NaiveTime};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::validate_uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddScheduleDto {
    pub organization: uuid::Uuid,
    pub branch: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub fee: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecurringType {
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

impl std::fmt::Display for RecurringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RecurringType::DAILY => "DAILY",
            RecurringType::WEEKLY => "WEEKLY",
            RecurringType::MONTHLY => "MONTHLY",
            RecurringType::YEARLY => "YEARLY",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleRecurringDto {
    pub organization: uuid::Uuid,
    pub branch: uuid::Uuid,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub recurring_type: Option<RecurringType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleDiscountDto {
    pub organization: uuid::Uuid,
    pub branch: uuid::Uuid,
    pub rate: Option<Decimal>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddScheduleParams {
    #[validate(custom(function = "validate_uuid"))]
    pub branch: String,
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 50, message = "Description is invalid"))]
    pub description: String,
    #[validate(range(min = 0.0, max = 10000.0, message = "Fee must be non-negative"))]
    pub fee: f64,
}
