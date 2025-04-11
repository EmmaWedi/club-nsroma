use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use sea_orm::prelude::Decimal;
use serde_json::json;

use crate::{
    app::{
        departments::{
            dtos::dto::{
                get_branch_departments, get_department, get_organization_departments,
                save_department, toggle_delete,
            },
            models::model::{AddDepartmentDto, AddDepartmentParams},
        },
        users::models::model::UserResponse,
    },
    libs::error,
    utils::{
        json_validator::{ValidatedJson, ValidatedPath},
        models::{HttpClientResponse, PathParamsModel, ResponseCode},
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
        .get::<Arc<UserResponse>>()
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

pub async fn get_departments_by_organization(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<UserResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "User not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let departments = get_organization_departments(model.organization_id, &state).await;

    if let Err(e) = departments {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Error Retrieving: {}", e),
            json!([]),
        )));
    }

    let res = departments.unwrap();

    if res.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Departments Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Departments Fetched Successfully".to_string(),
        json!(res),
    )))
}

pub async fn get_departments_by_branch(
    req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<UserResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "User not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = params.0;

    let departments = get_branch_departments(model.organization_id, data.id, &state).await;

    if let Err(e) = departments {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Error Retrieving: {}", e),
            json!([]),
        )));
    }

    let res = departments.unwrap();

    if res.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Departments Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Departments Fetched Successfully".to_string(),
        json!(res),
    )))
}

pub async fn get_department_detail(
    req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<UserResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "User not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = params.0;

    let department = get_department(data.id, model.organization_id, &state).await;

    if let Err(e) = department {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to Retrieve Department: {}", e),
            json!({}),
        )));
    }

    let res = department.unwrap();

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Department Retrieved Successfully".to_string(),
        json!(res),
    )))
}

pub async fn toggle_deletion(
    req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<UserResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "User not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = params.0;

    let result = toggle_delete(data.id, model.organization_id, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to Toggle Department: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Department Toggled Successfully".to_string(),
        json!({}),
    )))
}
