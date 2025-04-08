use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    libs::error,
    utils::{
        file_methods::{file_exists, read_file},
        shared::{get_media_by_id, parse_uuid},
    },
    AppState,
};

async fn req_read_file(
    _req: HttpRequest,
    id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let parsed_id = parse_uuid(&id);

    let media = get_media_by_id(parsed_id, &state).await;

    match media {
        Ok(model) => {
            let file_name = model.file_name.unwrap();
            let mime_type = model.mime_type.unwrap();
            let extension = mime_type.split('/').nth(1);
            let sub_path = model.file_path.unwrap();
            if file_exists(&sub_path, &file_name, &extension.unwrap()).await {
                match read_file(&sub_path, &file_name, &extension.unwrap()).await {
                    Ok(data) => Ok(HttpResponse::Ok().content_type(mime_type).body(data)),
                    Err(_) => Ok(HttpResponse::InternalServerError().body("Error reading file")),
                }
            } else {
                Ok(HttpResponse::NotFound().body("File not found"))
            }
        }
        _ => Ok(HttpResponse::NotFound().body("Media not found")),
    }
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/v1/media/{id}").route(web::get().to(req_read_file)));
}
