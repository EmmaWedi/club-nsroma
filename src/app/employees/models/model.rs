use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEmployeeDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: String,
    pub emp_num: String,
    pub gender: String,
    pub date_of_birth: NaiveDate
}