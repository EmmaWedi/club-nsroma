use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use base64::Engine;
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
        password::{encrypt_password, salt},
    },
    utils::{
        file_methods::save_file,
        json_validator::ValidatedJson,
        models::{HttpClientResponse, ImageUploadParams, ResponseCode, SaveMediaDto},
        shared::save_media_meta,
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
    let now_date = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();

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

    let extension = data.mime_type.split('/').nth(1);
    let sub_path = "identifications";

    let decoded = match base64::engine::general_purpose::STANDARD.decode(&data.img) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
                ResponseCode::Failed,
                "Invalid base64".to_string(),
                json!({}),
            )))
        }
    };
    
    let id = customer.unwrap().id;
    let file_name = format!("{}-{}", id.clone(), now_date);

    let save_result = save_file(
        sub_path,
        &file_name,
        extension.unwrap_or_default(),
        &decoded,
    )
    .await;

    if let Err(e) = save_result {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to save the image: {}", e),
            json!({}),
        )));
    }

    let media = SaveMediaDto {
        file_name: file_name.clone(),
        mime_type: data.mime_type.clone(),
        file_path: format!(
            "uploads/{}/{}.{}",
            sub_path,
            file_name,
            extension.unwrap_or_default()
        ),
        file_size: data.file_size.clone(),
        media_type: data.media_type.clone(),
        width: data.width,
        height: data.height,
    };

    let saved_media = save_media_meta(id, media, &state).await;

    if let Err(e) = saved_media {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Failed,
            format!("Failed to save image meta: {}", e),
            json!({}),
        )));
    }

    let id_info = AddCustomerIDDto {
        id_type: data.id_type.clone(),
        id_num: data.id_nun.clone(),
        id_img: saved_media.unwrap().last_insert_id.to_string(),
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
