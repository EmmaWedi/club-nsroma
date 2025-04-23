use actix_web::web;
use base64::Engine;
use sea_orm::{
    ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, PaginatorTrait, QueryFilter, Set,
};

use crate::AppState;

use super::{
    file_methods::{file_exists, save_file},
    models::{SaveMediaDto, SaveMediaFilesDto},
};

pub fn parse_uuid(id: &str) -> uuid::Uuid {
    uuid::Uuid::parse_str(id).expect("Invalid UUID string")
}

async fn save_media_meta(
    owner: uuid::Uuid,
    data: SaveMediaDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::medias::ActiveModel>, DbErr> {
    let extension = data.mime_type.split('/').nth(1);

    if !file_exists(&data.file_path, &data.file_name, &extension.unwrap()).await {
        return Err(DbErr::Custom("File does exist".to_string()));
    };

    let media_data = entity::medias::ActiveModel {
        owner_id: Set(owner),
        file_path: Set(Some(data.file_path)),
        mime_type: Set(Some(data.mime_type)),
        file_size: Set(Some(data.file_size)),
        file_name: Set(Some(data.file_name)),
        media_type: Set(Some(data.media_type)),
        width: Set(data.width),
        height: Set(data.height),
        ..Default::default()
    };

    let insertion = entity::medias::Entity::insert(media_data)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

pub async fn get_media_by_id(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::medias::Model, DbErr> {
    let media = entity::medias::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Media not found or is blocked".into()));

    media
}

pub async fn get_media_by_user(
    owner: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::medias::Model>, DbErr> {
    let medias = entity::medias::Entity::find()
        .filter(Condition::all().add(entity::medias::Column::OwnerId.eq(owner)))
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(medias)
}

pub async fn gen_num(
    model: &str,
    name: String,
    state: &web::Data<AppState>,
) -> Result<String, DbErr> {
    let count = match model {
        "EMP" => entity::employees::Entity::find()
            .count(state.pg_db.get_ref())
            .await
            .unwrap_or(0),
        "USR" => entity::users::Entity::find()
            .count(state.pg_db.get_ref())
            .await
            .unwrap_or(0),
        "CUS" => entity::customers::Entity::find()
            .count(state.pg_db.get_ref())
            .await
            .unwrap_or(0),
        _ => 0,
    };

    let trimmed = name.trim();

    let name_part = if trimmed.len() >= 3 {
        &trimmed[0..3]
    } else {
        &trimmed
    };

    let roll = count + 1;

    let counter = format!("{}{}{:04}", model, name_part.to_uppercase(), roll);

    Ok(counter)
}

pub async fn save_media_files(
    data: SaveMediaFilesDto,
    state: &web::Data<AppState>,
) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
    let now_date = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();

    let extension = data.mime_type.split('/').nth(1);

    let decoded = match base64::engine::general_purpose::STANDARD.decode(&data.data) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let file_name = format!("{}-{}", data.id.clone(), now_date);

    if let Err(e) = save_file(
        &data.dir,
        &file_name,
        extension.unwrap_or_default(),
        &decoded,
    )
    .await
    {
        return Err(Box::new(e));
    }

    let media = SaveMediaDto {
        file_name: file_name.clone(),
        mime_type: data.mime_type.clone(),
        file_path: format!(
            "uploads/{}/{}.{}",
            data.dir,
            file_name,
            extension.unwrap_or_default()
        ),
        file_size: data.size,
        media_type: data.media_type,
        width: data.width,
        height: data.height,
    };

    let saved_media = save_media_meta(data.id, media, &state).await;

    if let Err(e) = saved_media {
        return Err(Box::new(e));
    }

    Ok(saved_media.unwrap().last_insert_id)
}
