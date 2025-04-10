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
        "image/png" => Ok(()),
        _ => Err(ValidationError::new("Invalid mime type")),
    }
}

// pub fn required_str(v: &str, name: &str) -> Result<String, error::Error> {
//     let v = v.to_string();

//     if v.chars().count() == 0 {
//         return Err(error::new_error(
//             1002,
//             &format!("{} is required", name)[..],
//             422,
//         ));
//     }

//     Ok(v)
// }

// pub fn email(v: &str, name: &str) -> Result<String, error::Error> {
//     let res_str = v.to_string();

//     let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

//     if !re.is_match(v) {
//         return Err(error::new_error(
//             1002,
//             &format!("{} validation failed", name)[..],
//             422,
//         ));
//     }

//     Ok(res_str)
// }

// pub fn mobile(v: &str, name: &str) -> Result<String, error::Error> {
//     let re = Regex::new(r"^[0-9]{10}$").unwrap();

//     if !re.is_match(v) {
//         return Err(error::new_error(
//             1002,
//             &format!("{} validation failed", name)[..],
//             422,
//         ));
//     }

//     let res_str = v.to_string();

//     Ok(res_str)
// }

// pub fn uuid(v: &str, name: &str) -> Result<uuid::Uuid, error::Error> {
//     let re = Regex::new(r"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap();

//     if !re.is_match(&(v.to_uppercase())[..]) {
//         return Err(error::new_error(
//             1002,
//             &format!("{} is invalid", name)[..],
//             422,
//         ));
//     }

//     let res_str = uuid::Uuid::parse_str(v)
//         .map_err(|_| error::new_error(1002, &format!("{} is not a valid UUID", name), 422))?;

//     Ok(res_str)
// }

// pub fn date(v: &str, name: &str) -> Result<NaiveDate, error::Error> {
//     let date_format = "%Y-%m-%d";

//     let parsed_date = match NaiveDate::parse_from_str(v, date_format) {
//         Ok(date) => date,
//         Err(_) => {
//             return Err(error::new_error(
//                 1002,
//                 &format!("{} validation failed", name),
//                 422,
//             ));
//         }
//     };

//     Ok(parsed_date)
// }

// pub fn not_none<T>(v: Option<T>, name: &str) -> Result<(), error::Error> {

//     if let None = v {
//         return Err(error::new_error(1002, &format!("{} cannot be empty", name)[..], 422));
//     }

//     Ok(())
// }
