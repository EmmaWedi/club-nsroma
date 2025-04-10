use base64::Engine;
use chrono::{Datelike, NaiveDateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
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

    if birth_date >= &now {
        return Err(ValidationError::new("Birth date must be in the past"));
    }

    let age = {
        let now_date = Utc::now().date_naive();
        let birth = birth_date.date();
        let mut age = now_date.year() - birth.year();

        if now_date.ordinal() < birth.ordinal() {
            age -= 1;
        }

        age
    };

    if age < 18 {
        return Err(ValidationError::new("Must be at least 18 years old"));
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
