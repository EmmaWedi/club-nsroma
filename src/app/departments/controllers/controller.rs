use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use sea_orm::prelude::Decimal;
use serde_json::json;

use crate::{
    app::{
        departments::{
            dtos::dto::save_department,
            models::model::{AddDepartmentDto, AddDepartmentParams},
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

pub async fn add_department(
    req: HttpRequest,
    payload: ValidatedJson<AddDepartmentParams>,
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

    let branch_uuid = uuid::Uuid::parse_str(&data.branch.clone()).unwrap_or_default();
    let daily = Decimal::from_f64_retain(data.daily_rate).unwrap_or_default();
    let hourly = Decimal::from_f64_retain(data.hourly_rate).unwrap_or_default();

    let department = AddDepartmentDto {
        name: data.name.clone(),
        description: data.description.clone(),
        branch: branch_uuid,
        organization: model.organization_id,
        for_all: false,
        employee_num: data.num_of_employee,
        leave_days: data.leave_days,
        daily_rate: daily,
        hourly_rate: hourly,
    };

    let result = save_department(department, &state).await;

    match result {
        Ok(res) => Ok(HttpResponse::Created().json(HttpClientResponse::new(
            ResponseCode::Success,
            "Saving Successful".to_string(),
            json!(res.last_insert_id),
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Adding Organization: {}", e),
                json!({}),
            )),
        ),
    }
}
