use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::customers::{
        dtos::dto::{
            get_customer_by_contact, get_customer_details, get_customer_full,
            get_customer_with_auth, save_customer, save_id_info, update_customer_details,
        },
        models::model::{
            AddCustomerDto, AddCustomerIDDto, CustomerResponse, SignInCustomerParams,
            SignUpCustomerParams, UpdateCustomerDto, UpdateCustomerParams,
        },
    },
    libs::{
        error,
        jwt::create_jwt,
        password::encrypt_password,
    },
    utils::{
        json_validator::ValidatedJson,
        models::{
            HttpClientResponse, ImageUploadParams, ResponseCode, SaveMediaFilesDto,
        },
        shared::save_media_files,
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

    let salt = uuid::Uuid::new_v4();

    let signupdata = AddCustomerDto {
        username: data.username,
        contact: data.phone,
        password: encrypt_password(&data.password, &salt).await,
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

pub async fn signin_customer(
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
        .get::<Arc<CustomerResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "Customer not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = &model.session {
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
        json!(customer.unwrap()),
    )))
}

pub async fn update_details(
    req: HttpRequest,
    payload: ValidatedJson<UpdateCustomerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<CustomerResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "Customer not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let data = payload.0;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = &model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let upd_data = UpdateCustomerDto {
        email: data.email,
        username: data.username,
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

pub async fn upload_image(
    req: HttpRequest,
    payload: ValidatedJson<ImageUploadParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let model = req
        .extensions()
        .get::<Arc<CustomerResponse>>()
        .cloned()
        .ok_or(error::Error {
            message: "Customer not found".to_string(),
            code: 2001,
            status: 500,
        })?;

    let mut session_id = uuid::Uuid::nil();

    if let Some(session_uuid) = &model.session {
        if let Ok(s_uuid) = uuid::Uuid::parse_str(&session_uuid) {
            session_id = s_uuid
        }
    }

    let customer = get_customer_full(session_id, &state).await;

    if let Err(e) = customer {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Could not find customer: {}", e),
            json!({}),
        )));
    }

    let data = payload.0;

    let media = SaveMediaFilesDto {
        id: customer.unwrap().id,
        dir: "identities".to_string(),
        data: data.img,
        mime_type: data.mime_type,
        media_type: data.media_type,
        size: data.file_size,
        width: data.width,
        height: data.height,
    };

    let saver = save_media_files(media, &state).await;

    if let Err(e) = saver {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Could not find customer: {}", e),
            json!({}),
        )));
    }

    let id_info = AddCustomerIDDto {
        id_type: data.id_type,
        id_num: data.id_nun,
        id_img: saver.unwrap().to_string(),
        is_verified: true,
    };

    if let Err(e) = save_id_info(session_id, id_info, &state).await {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to save image info: {}", e),
            json!({}),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Success".to_string(),
        json!({}),
    )))
}
