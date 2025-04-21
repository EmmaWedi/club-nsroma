use std::{str::FromStr, sync::Arc};

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::{
        employees::models::model::EmployeeResponse,
        schedules::{
            dtos::dto::{
                get_schedule, get_schedules, get_schedules_by_branch, get_schedules_by_org,
                save_schedule, set_student_schedule, toggle_discount, toggle_recurring,
            },
            models::model::{
                AddScheduleDto, AddScheduleParams, DiscountParams, RecurringParams, RecurringType,
                StudentParams, ToggleDiscountDto, ToggleRecurringDto,
            },
        },
    },
    libs::error::Error,
    utils::{
        json_validator::{ValidatedJson, ValidatedPath},
        models::{HttpClientResponse, PathParamsModel, ResponseCode},
    },
    AppState,
};

pub async fn create_schedule(
    req: HttpRequest,
    payload: ValidatedJson<AddScheduleParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let model = req
        .extensions()
        .get::<Arc<EmployeeResponse>>()
        .cloned()
        .ok_or(Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;

    let schedule = AddScheduleDto {
        organization: model.organization_id,
        branch: data.branch,
        name: data.name,
        description: data.description,
        fee: data.fee,
    };

    let created_schedule = save_schedule(schedule, &state).await;

    match created_schedule {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "Scheule Created Successful".to_string(),
            json!(res.last_insert_id),
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Creating Schedule: {}", e),
                json!({}),
            )),
        ),
    }
}

pub async fn get_active_schedules(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let results = get_schedules(&state).await;

    if let Err(e) = results {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    };

    let schedules = results.unwrap();

    if schedules.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Departments Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedules Fetched Successfully".to_string(),
        json!(schedules),
    )))
}

pub async fn get_organization_schedules(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let data = params.0;

    let results = get_schedules_by_org(data.id, &state).await;

    if let Err(e) = results {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    };

    let schedules = results.unwrap();

    if schedules.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Departments Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedules Fetched Successfully".to_string(),
        json!(schedules),
    )))
}

pub async fn get_branch_schedules(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let model = req
        .extensions()
        .get::<Arc<EmployeeResponse>>()
        .cloned()
        .ok_or(Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let results = get_schedules_by_branch(model.organization_id, model.branch_id, &state).await;

    if let Err(e) = results {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    };

    let schedules = results.unwrap();

    if schedules.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Departments Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedules Fetched Successfully".to_string(),
        json!(schedules),
    )))
}

pub async fn get_schedule_details(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let data = params.0;

    let result = get_schedule(data.id, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    };

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedule Fetched Successfully".to_string(),
        json!(result.unwrap()),
    )))
}

pub async fn tog_occurance(
    req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    payload: ValidatedJson<RecurringParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let model = req
        .extensions()
        .get::<Arc<EmployeeResponse>>()
        .cloned()
        .ok_or(Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;
    let path_params = params.0;

    let tog_data = ToggleRecurringDto {
        organization: model.organization_id,
        branch: model.branch_id,
        start_date: data.start_date,
        end_date: data.end_date,
        start_time: data.start_time,
        end_time: data.end_time,
        recurring_type: data
            .recurring
            .as_deref()
            .and_then(|s| RecurringType::from_str(s).ok()),
    };

    let result = toggle_recurring(path_params.id, tog_data, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedule Updated Successfully".to_string(),
        json!({}),
    )))
}

pub async fn upd_discount(
    req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    payload: ValidatedJson<DiscountParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let model = req
        .extensions()
        .get::<Arc<EmployeeResponse>>()
        .cloned()
        .ok_or(Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;
    let path_params = params.0;

    let discount_data = ToggleDiscountDto {
        organization: model.organization_id,
        branch: data.branch,
        rate: Some(data.rate),
    };

    let result = toggle_discount(path_params.id, discount_data, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedule Updated Successfully".to_string(),
        json!({}),
    )))
}

pub async fn set_student(
    req: HttpRequest,
    params: ValidatedPath<StudentParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let model = req
        .extensions()
        .get::<Arc<EmployeeResponse>>()
        .cloned()
        .ok_or(Error {
            message: "Employee not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = params.0;

    let result = set_student_schedule(data.id, data.branch, model.organization_id, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Schedule not found: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Schedule Updated Successfully".to_string(),
        json!({}),
    )))
}
