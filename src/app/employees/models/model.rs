use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::libs::validator::{
    validate_birth_date, validate_contact, validate_gender, validate_marital, validate_uuid,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEmployeeDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: String,
    pub gender: String,
    pub date_of_birth: NaiveDate,
    pub marital_status: String,
    pub branch: uuid::Uuid,
    pub organization: uuid::Uuid,
    pub department: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct EmployeeResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: String,
    pub employee_number: Option<String>,
    pub address: String,
    pub gender: String,
    pub date_of_birth: NaiveDate,
    pub marital_status: String,
    pub organization_id: uuid::Uuid,
    pub branch_id: uuid::Uuid,
    pub department_id: uuid::Uuid,
    pub identification_type: Option<String>,
    pub identification_number: Option<String>,
    pub identification_image_id: Option<String>,
    pub tax_identification_number: Option<String>,
    pub is_deleted: bool,
    pub is_active: bool,
    pub is_booked_on: Option<bool>,
    pub employee_start_date: NaiveDate,
    pub employee_end_date: Option<NaiveDate>,
    pub employee_status: String,
    pub role_permissions: Option<uuid::Uuid>,
    pub session: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<entity::employees::Model> for EmployeeResponse {
    fn from(employee: entity::employees::Model) -> Self {
        Self {
            first_name: employee.first_name,
            last_name: employee.last_name,
            email: employee.email,
            contact: employee.contact,
            employee_number: Some(employee.employee_number),
            address: employee.address,
            gender: employee.gender,
            date_of_birth: employee.date_of_birth,
            marital_status: employee.marital_status,
            organization_id: employee.organization_id,
            branch_id: employee.branch_id,
            department_id: employee.department_id,
            identification_type: employee.identification_type,
            identification_number: employee.identification_number,
            identification_image_id: employee.identification_image_id,
            tax_identification_number: employee.tax_identification_number,
            is_deleted: employee.is_deleted,
            is_active: employee.is_active,
            is_booked_on: employee.is_booked_on,
            employee_start_date: employee.employee_start_date,
            employee_end_date: employee.employee_end_date,
            employee_status: employee.employee_status,
            role_permissions: employee.role_permissions,
            session: employee.session,
            created_at: employee.created_at.naive_utc().into(),
            updated_at: employee.updated_at.naive_utc().into(),
            deleted_at: employee.deleted_at.map(|dt| dt.naive_utc().into()),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddEmployeeParams {
    #[validate(length(min = 3, max = 20, message = "First name is invalid"))]
    pub first_name: String,
    #[validate(length(min = 3, max = 20, message = "Last name is invalid"))]
    pub last_name: String,
    #[validate(custom(function = "validate_contact"))]
    pub contact: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(custom(function = "validate_gender"))]
    pub gender: String,
    #[validate(custom(function = "validate_birth_date"))]
    pub birth_date: NaiveDateTime,
    #[validate(custom(function = "validate_marital"))]
    pub marital_status: String,
    #[validate(custom(function = "validate_uuid"))]
    pub branch: String,
    #[validate(custom(function = "validate_uuid"))]
    pub department: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeDetailsResponse {
    pub employee: EmployeeResponse,
    pub organization: entity::organizations::Model,
    pub branch: entity::branches::Model,
    pub department: entity::departments::Model,
}