use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::users::{
        dtos::dto::{get_user_with_auth, get_user_with_organization},
        models::model::{DetailsResponse, SignInRequestModel, UserResponse},
    },
    libs::{error, jwt::create_jwt},
    utils::{
        json_validator::ValidatedJson,
        models::{HttpClientResponse, ResponseCode},
    },
    AppState,
};

pub async fn signin_user(
    _req: HttpRequest,
    payload: ValidatedJson<SignInRequestModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = payload.0;

    let result = get_user_with_auth(data.phone.clone(), data.password.clone(), &state).await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::Ok().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Wrong Credentials: {}", e),
                json!({}),
            )),
        );
    };

    let user = result.unwrap();

    let gen_token = create_jwt(user.session.unwrap_or_default(), &state).await;

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Sign In Successful".to_string(),
        json!(gen_token.token),
    )))
}

pub async fn get_user_details(
    req: HttpRequest,
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

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let results = get_user_with_organization(session_id, &state).await;

    if let Err(e) = results {
        return Ok(
            HttpResponse::Ok().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Wrong Credentials: {}", e),
                json!({}),
            )),
        );
    };

    let (user, organization) = results.unwrap();

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Success".to_string(),
        json!(DetailsResponse { user, organization }),
    )))
}
