use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::users::{dtos::dto::get_user_with_auth, models::model::SignInRequestModel},
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
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
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
