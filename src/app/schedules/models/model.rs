use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::{
    validate_naive_date, validate_percent_range, validate_recurring_type, validate_val_range,
};

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

impl FromStr for RecurringType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DAILY" => Ok(RecurringType::DAILY),
            "WEEKLY" => Ok(RecurringType::WEEKLY),
            "MONTHLY" => Ok(RecurringType::MONTHLY),
            "YEARLY" => Ok(RecurringType::YEARLY),
            _ => Err(()),
        }
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
#[serde(deny_unknown_fields)]
pub struct AddScheduleParams {
    pub branch: uuid::Uuid,
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 50, message = "Description is invalid"))]
    pub description: String,
    #[validate(custom(function = "validate_val_range"))]
    pub fee: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct DiscountParams {
    #[validate(custom(function = "validate_percent_range"))]
    pub rate: Decimal,
    pub branch: uuid::Uuid,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct RecurringParams {
    #[validate(custom(function = "validate_naive_date"))]
    pub start_date: Option<NaiveDate>,
    #[validate(custom(function = "validate_naive_date"))]
    pub end_date: Option<NaiveDate>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    #[validate(custom(function = "validate_recurring_type"))]
    pub recurring: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct StudentParams {
    pub branch: uuid::Uuid,
    pub id: uuid::Uuid,
}
