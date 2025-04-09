use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::customers::{
        dtos::dto::{
            get_customer_by_contact, get_customer_details, get_customer_with_auth, save_customer,
            update_customer_details,
        },
        models::model::{
            AddCustomerDto, CustomerResponse, SignInCustomerParams, SignUpCustomerParams,
            UpdateCustomerDto, UpdateCustomerParams,
        },
    },
    libs::{
        error,
        jwt::create_jwt,
        password::{encrypt_password, salt},
    },
    utils::{
        json_validator::ValidatedJson,
        models::{HttpClientResponse, ResponseCode},
    },
    AppState,
};

pub async fn signup(
    _req: HttpRequest,
    payload: ValidatedJson<SignUpCustomerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = payload.0;

    if let Ok(customer) = get_customer_by_contact(data.phone.clone(), &state).await {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Customer With Phone {} Exists", customer.contact),
            json!({}),
        )));
    }

    let salt = salt();

    let signupdata = AddCustomerDto {
        username: data.username.clone(),
        contact: data.phone.clone(),
        password: encrypt_password(&data.password, &salt),
        salt: salt.to_string(),
    };

    let result = save_customer(signupdata, &state).await;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "Customer Added Successful".to_string(),
            json!(res.last_insert_id),
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Adding Customer: {}", e),
                json!({}),
            )),
        ),
    }
}

pub async fn sigin_customer(
    _req: HttpRequest,
    payload: ValidatedJson<SignInCustomerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = payload.0;

    let result = get_customer_with_auth(data.phone.clone(), data.password.clone(), &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Wrong Credentials: {}", e),
            json!({}),
        )));
    }

    let customer = result.unwrap();

    let gen_token = create_jwt(customer.session.unwrap_or_default(), &state).await;

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Sign In Successful".to_string(),
        json!(gen_token.token),
    )))
}

pub async fn get_details(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<CustomerResponse>()
        .cloned()
        .ok_or(error::Error {
            message: "Customer not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let customer = get_customer_details(session_id, &state).await;

    if let Err(e) = customer {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Could not get customer: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Success".to_string(),
        json!({}),
    )))
}

pub async fn update_details(
    req: HttpRequest,
    payload: ValidatedJson<UpdateCustomerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<CustomerResponse>()
        .cloned()
        .ok_or(error::Error {
            message: "Customer not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let upd_data = UpdateCustomerDto {
        email: data.email.clone(),
        username: data.username.clone(),
    };

    let result = update_customer_details(session_id, upd_data, &state).await;

    if let Err(e) = result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Could not update customer: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Success".to_string(),
        json!(result.unwrap()),
    )))
}
