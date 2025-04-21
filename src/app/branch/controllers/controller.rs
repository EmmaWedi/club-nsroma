use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::{
        branch::{
            dtos::dto::{get_organization_branches, save_branch},
            models::model::{AddBranchDto, AddBranchParams},
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

pub async fn add_branch(
    req: HttpRequest,
    payload: ValidatedJson<AddBranchParams>,
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

    let branch = AddBranchDto {
        name: data.name,
        gps: data.gps,
        location: data.location,
        contact: data.contact,
        email: data.email,
        organization: model.organization_id,
        country: data.country,
    };

    let result = save_branch(branch, &state).await;

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

pub async fn get_branches_for_organization(
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

    let branches = get_organization_branches(model.organization_id, &state).await;

    if let Err(e) = branches {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Error Retrieving: {}", e),
            json!([]),
        )));
    }

    let res = branches.unwrap();

    if res.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Branches Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Branches Fetched Successfully".to_string(),
        json!(res),
    )))
}
