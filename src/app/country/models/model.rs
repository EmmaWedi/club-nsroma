use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCountryDto {
    pub name: String,
    pub call_code: String,
    pub currency_code: String,
    pub currency: String,
    pub iso_code: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddCountryParamsModel {
    #[validate(length(min = 3, max = 20, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 1, max = 5, message = "Code is invalid"))]
    pub call_code: String,
    #[validate(length(min = 2, max = 5, message = "Currency code is invalid"))]
    pub currency_code: String,
    #[validate(length(min = 1, max = 20, message = "Currency is invalid"))]
    pub currency: String,
    #[validate(length(min = 2, max = 5, message = "Iso is invalid"))]
    pub iso_code: String,
}