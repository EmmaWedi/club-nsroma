use base64::Engine;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::prelude::Decimal;
use validator::ValidationError;

static CONTACT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]{10}$").unwrap());

pub fn validate_contact(contact: &str) -> Result<(), ValidationError> {
    if !CONTACT_REGEX.is_match(contact) {
        return Err(ValidationError::new("Contact must be exactly 10 digits"));
    }

    Ok(())
}

pub fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::parse_str(uuid_str).is_err() {
        return Err(ValidationError::new("Invalid UUID format"));
    }
    Ok(())
}

pub fn validate_gender(gender: &str) -> Result<(), ValidationError> {
    match gender {
        "MALE" | "FEMALE" => Ok(()),
        _ => Err(ValidationError::new("Invalid gender format")),
    }
}

pub fn validate_marital(marital: &str) -> Result<(), ValidationError> {
    match marital {
        "SINGLE" | "MARRIED" | "DIVORCED" | "WIDOWED" => Ok(()),
        _ => Err(ValidationError::new("Invalid marital status")),
    }
}

pub fn validate_id_type(marital: &str) -> Result<(), ValidationError> {
    match marital {
        "NATIONALID" | "DRIVERLICENCE" | "PASSPORT" => Ok(()),
        _ => Err(ValidationError::new("Invalid id type status")),
    }
}

pub fn validate_birth_date(birth_date: &NaiveDateTime) -> Result<(), ValidationError> {
    let now = Utc::now().naive_utc();

    if *birth_date >= now {
        let mut err = ValidationError::new("birth_date_in_future");
        err.message = Some("Birth date must be in the past".into());
        return Err(err);
    }

    let now_date = now.date();
    let birth = birth_date.date();
    let mut age = now_date.year() - birth.year();

    if now_date.ordinal() < birth.ordinal() {
        age -= 1;
    }

    if age < 18 {
        let mut err = ValidationError::new("underage");
        err.message = Some("Must be at least 18 years old".into());
        return Err(err);
    }

    Ok(())
}

pub fn validate_mime_type(mime_type: &str) -> Result<(), ValidationError> {
    match mime_type {
        "image/png" | "image/jpg" => Ok(()),
        _ => Err(ValidationError::new("Invalid mime type")),
    }
}

pub fn validate_base64_file_size(base64_data: &str) -> Result<(), ValidationError> {
    const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5 MB
    let decoded = base64::engine::general_purpose::STANDARD.decode(base64_data);
    if let Ok(data) = decoded {
        if data.len() > MAX_FILE_SIZE {
            return Err(ValidationError::new("file too large"));
        }
        Ok(())
    } else {
        Err(ValidationError::new("invalid base64"))
    }
}

pub fn validate_naive_date_rest(date: &NaiveDate) -> Result<(), ValidationError> {
    let today = chrono::Local::now().naive_local().date();

    if *date > today {
        let mut err = ValidationError::new("date_in_future");
        err.message = Some("Date cannot be in the future".into());
        return Err(err);
    }

    Ok(())
}

pub fn validate_naive_date(date: &NaiveDate) -> Result<(), ValidationError> {
    if *date < chrono::Utc::now().naive_utc().date() {
        return Err(ValidationError::new("date_cannot_be_in_past"));
    }

    Ok(())
}


pub fn validate_naive_time_rest(time: &NaiveTime) -> Result<(), ValidationError> {
    let now = chrono::Local::now().naive_local().time();

    if *time > now {
        let mut err = ValidationError::new("time_in_future");
        err.message = Some("Time cannot be in the future".into());
        return Err(err);
    }

    Ok(())
}

pub fn validate_naive_datetime(datetime_str: &str) -> Result<(), ValidationError> {
    match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_datetime_format")),
    }
}

pub fn validate_recurring_type(recurring: &str) -> Result<(), ValidationError> {
    match recurring {
        "DAILY" | "WEEKLY" | "MONTHLY" | "YEARLY" => Ok(()),
        _ => Err(ValidationError::new("Invalid id type status")),
    }
}

pub fn validate_val_range(fee: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::from_f64_retain(0.0).unwrap();
    let max = Decimal::from_f64_retain(10000.0).unwrap();

    if fee < &min || fee > &max {
        let mut err = ValidationError::new("fee_out_of_range");
        err.message = Some("Fee must be between 0 and 10,000.".into());
        return Err(err);
    }

    Ok(())
}

pub fn validate_percent_range(fee: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::from_f64_retain(0.1).unwrap();
    let max = Decimal::from_f64_retain(100.0).unwrap();

    if fee < &min || fee > &max {
        let mut err = ValidationError::new("fee_out_of_range");
        err.message = Some("Fee must be between 0 and 10,000.".into());
        return Err(err);
    }

    Ok(())
}