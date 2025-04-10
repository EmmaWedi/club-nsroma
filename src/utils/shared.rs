use actix_web::web;
use sea_orm::{
    ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, PaginatorTrait, QueryFilter, Set,
};

use crate::AppState;

use super::{file_methods::file_exists, models::SaveMediaDto};

pub fn parse_uuid(id: &str) -> uuid::Uuid {
    uuid::Uuid::parse_str(id).expect("Invalid UUID string")
}

pub async fn save_media_meta(
    owner: uuid::Uuid,
    data: SaveMediaDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::medias::ActiveModel>, DbErr> {
    let extension = data.mime_type.split('/').nth(1);

    if file_exists(&data.file_path, &data.file_name, &extension.unwrap()).await {
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

//get media by id
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

//get media by user
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

    // let counter = format!("{:012}{}", count, name_part);
    let counter = format!("{}{}{}", model, name_part, count);

    Ok(counter)
}
