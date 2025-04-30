use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::{
        organization::{
            dtos::dto::{get_organization, get_organizations, save_organization},
            models::model::{AddOrganizationDto, AddOrganizationParams, ReturnCredentialsModel},
        },
        users::{
            dtos::dto::{get_user_by_contact, save_users},
            models::model::AddUserDto,
        },
    },
    libs::{error, jwt::gen_string, password::encrypt_password},
    utils::{
        json_validator::{ValidatedJson, ValidatedPath},
        models::{HttpClientResponse, PathParamsModel, ResponseCode},
    },
    AppState,
};

pub async fn add_organization(
    _req: HttpRequest,
    payload: ValidatedJson<AddOrganizationParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = payload.0;

    if let Ok(user) = get_user_by_contact(data.contact.clone(), &state).await {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("User With Email {} Exists", user.contact),
            json!({}),
        )));
    }

    let organizationdto = AddOrganizationDto {
        name: data.name,
        post_code: data.gps,
        location: data.location,
        country: data.country,
    };

    let result = save_organization(organizationdto, &state).await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Adding Organization: {}", e),
                json!({}),
            )),
        );
    };

    let organization = result.unwrap();

    let salt = uuid::Uuid::new_v4();
    let password = gen_string(14);

    let userdto = AddUserDto {
        first_name: data.first_name,
        last_name: data.last_name,
        organization: organization.last_insert_id,
        contact: data.contact.clone(),
        email: data.email.unwrap_or_default(),
        salt: salt.to_string(),
        session: uuid::Uuid::new_v4().to_string(),
        password: encrypt_password(&password, &salt).await,
    };

    if let Err(e) = save_users(userdto, &state).await {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Saving User: {}", e),
                json!({}),
            )),
        );
    }

    Ok(HttpResponse::Created().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Organization Added Successfully".to_string(),
        json!(ReturnCredentialsModel {
            phone: data.contact,
            password
        }),
    )))
}

pub async fn get_active_organizations(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let results = get_organizations(&state).await;

    if let Err(e) = results {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Fetching Organizations: {}", e),
                json!([]),
            )),
        );
    }

    let res = results.unwrap();

    if res.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No Organizations Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Organizations Fetched Successfully".to_string(),
        json!(res),
    )))
}

pub async fn get_organization_details(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = params.0;

    let result = get_organization(data.id, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to Retrieve Organization: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Organization Retrieved Successfully".to_string(),
        json!(result.unwrap()),
    )))
}
