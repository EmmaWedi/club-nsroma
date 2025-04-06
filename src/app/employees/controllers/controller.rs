use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::{
        employees::{
            dtos::dto::{get_employee_details_comp, save_employee},
            models::model::{
                AddEmployeeDto, AddEmployeeParams, EmployeeDetailsResponse, EmployeeResponse,
            },
        },
        users::models::model::UserResponse,
    },
    libs::error,
    utils::{
        json_validator::ValidatedJson,
        models::{HttpClientResponse, ResponseCode},
    },
    AppState,
};

pub async fn add_employee(
    req: HttpRequest,
    payload: ValidatedJson<AddEmployeeParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<UserResponse>()
        .cloned()
        .ok_or(error::Error {
            message: "User not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;

    let employee = AddEmployeeDto {
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        email: data.email.clone(),
        contact: data.contact.clone(),
        gender: data.gender.clone(),
        date_of_birth: data.birth_date.into(),
        marital_status: data.marital_status.clone(),
        branch: uuid::Uuid::parse_str(&data.branch).unwrap_or_default(),
        organization: model.organization_id,
        department: uuid::Uuid::parse_str(&data.department).unwrap_or_default(),
    };

    let result = save_employee(employee, &state).await;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "Employee Added Successful".to_string(),
            json!(res.last_insert_id),
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Adding Employee: {}", e),
                json!({}),
            )),
        ),
    }
}

pub async fn emp_details(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<EmployeeResponse>()
        .cloned()
        .ok_or(error::Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let results = get_employee_details_comp(session_id, &state).await;

    if let Err(e) = results {
        return Ok(
            HttpResponse::Ok().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Employee not found: {}", e),
                json!({}),
            )),
        );
    };

    let (employee, organization, branch, department) = results.unwrap();

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Success".to_string(),
        json!(EmployeeDetailsResponse {
            employee,
            organization,
            branch,
            department
        }),
    )))
}
