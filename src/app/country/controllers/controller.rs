use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::country::{
        dtos::dto::{get_countries, get_country, save_country, toggle_active, toggle_deletion},
        models::model::{AddCountryDto, AddCountryParamsModel},
    },
    libs::error,
    utils::{
        json_validator::{ValidatedJson, ValidatedPath},
        models::{HttpClientResponse, PathParamsModel, ResponseCode},
    },
    AppState,
};

pub async fn add_country(
    _req: HttpRequest,
    payload: ValidatedJson<AddCountryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = &payload.0;

    let country = AddCountryDto {
        name: data.name.clone(),
        call_code: data.call_code.clone(),
        iso_code: data.iso_code.clone(),
        currency: data.currency.clone(),
        currency_code: data.currency_code.clone(),
    };

    let result = save_country(country, &state).await;

    match result {
        Ok(res) => Ok(HttpResponse::Created().json(HttpClientResponse::new(
            ResponseCode::Success,
            "Saving Successful".to_string(),
            json!(res.last_insert_id),
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Adding Country: {}", e),
                json!({}),
            )),
        ),
    }
}

pub async fn get_active_countries(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let results = get_countries(&state).await;

    if let Err(e) = results {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Error Fetching Countries: {}", e),
                json!([]),
            )),
        );
    }

    let res = results.unwrap();

    if res.is_empty() {
        return Ok(HttpResponse::Ok().json(HttpClientResponse::new(
            ResponseCode::Success,
            "No countries Found".to_string(),
            json!([]),
        )));
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Countries Fetched Successfully".to_string(),
        json!(res),
    )))
}

pub async fn toggle_activeness(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = &params.0;

    let result = toggle_active(data.id, &state).await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Failed to Update Country: {}", e),
                json!({}),
            )),
        );
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Country Updated Successfully".to_string(),
        json!({}),
    )))
}

pub async fn toggle_delete(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = &params.0;

    let result = toggle_deletion(data.id, &state).await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Failed to Delete Country: {}", e),
                json!({}),
            )),
        );
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Country Deleted Successfully".to_string(),
        json!({}),
    )))
}

pub async fn get_country_details(
    _req: HttpRequest,
    params: ValidatedPath<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let data = &params.0;

    let result = get_country(data.id, &state).await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse::new(
                ResponseCode::Failed,
                format!("Failed to Retrieve Country: {}", e),
                json!({}),
            )),
        );
    }

    Ok(HttpResponse::Ok().json(HttpClientResponse::new(
        ResponseCode::Success,
        "Country Retrieved Successfully".to_string(),
        json!(result.unwrap()),
    )))
}
