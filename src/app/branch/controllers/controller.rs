use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::branch::{
        dtos::dto::save_branch,
        models::model::{AddBranchDto, AddBranchParams},
    },
    libs::error,
    utils::{
        json_validator::ValidatedJson,
        models::{HttpClientResponse, ResponseCode},
    },
    AppState,
};

pub async fn add_branch(
    _req: HttpRequest,
    payload: ValidatedJson<AddBranchParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = payload.0;

    let branch = AddBranchDto {
        name: data.name.clone(),
        gps: data.gps.clone(),
        location: data.location.clone(),
        contact: data.contact.clone(),
        email: data.email.clone(),
        organization: data.organization.clone(),
        country: data.country.clone(),
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
