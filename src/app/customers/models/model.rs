use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomerDto {
    pub username: String,
    pub contact: String,
    pub customer_number: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomerDto {
    pub email: Option<String>,
    pub username: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct CustomerResponse {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub contact: String,
    pub email: Option<String>,
    pub is_student: bool,
    pub student_id_number: Option<String>,
    pub student_id_image_id: Option<String>,
    pub customer_number: String,
    pub identification_type: Option<String>,
    pub identification_number: Option<String>,
    pub identification_image_id: Option<String>,
    pub is_blocked: bool,
    pub is_deleted: bool,
    pub blocked_reason: Option<String>,
    pub is_banned: bool,
    pub session: Option<String>,
    pub username: Option<String>,
    pub student_end_year: Option<i32>,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<entity::customers::Model> for CustomerResponse {
    fn from(customer: entity::customers::Model) -> Self {
        Self {
            first_name: customer.first_name,
            last_name: customer.last_name,
            contact: customer.contact,
            email: customer.email,
            is_student: customer.is_student,
            student_id_number: customer.student_id_number,
            student_id_image_id: customer.student_id_image_id,
            customer_number: customer.customer_number,
            identification_type: customer.identification_type,
            identification_number: customer.identification_number,
            identification_image_id: customer.identification_image_id,
            is_blocked: customer.is_blocked,
            is_deleted: customer.is_deleted,
            blocked_reason: customer.blocked_reason,
            is_banned: customer.is_banned,
            session: customer.session,
            username: customer.username,
            student_end_year: customer.student_end_year,
            date_of_birth: customer.date_of_birth,
            created_at: customer.created_at.naive_utc().into(),
            updated_at: customer.updated_at.naive_utc().into(),
            deleted_at: customer.deleted_at.map(|dt| dt.naive_utc().into()),
        }
    }
}
