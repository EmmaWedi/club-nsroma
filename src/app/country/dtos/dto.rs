use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{app::country::models::model::AddCountryDto, AppState};

pub async fn save_country(
    data: AddCountryDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::countries::ActiveModel>, DbErr> {
    let country = entity::countries::ActiveModel {
        name: Set(data.name),
        call_code: Set(Some(data.call_code)),
        currency_code: Set(Some(data.currency_code)),
        currency: Set(Some(data.currency)),
        iso_code: Set(Some(data.iso_code)),
        is_active: Set(true),
        is_deleted: Set(false),
        ..Default::default()
    };

    let insertion = entity::countries::Entity::insert(country)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

pub async fn get_countries(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::countries::Model>, DbErr> {
    let countries = entity::countries::Entity::find()
        .filter(
            Condition::all()
                .add(entity::countries::Column::IsActive.eq(true))
                .add(entity::countries::Column::IsDeleted.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(countries)
}

pub async fn get_country(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::countries::Model, DbErr> {
    let country = entity::countries::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::countries::Column::IsActive.eq(true))
                .add(entity::countries::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Country not found".into()));

    country
}

pub async fn get_all_countries(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::countries::Model>, DbErr> {
    let countries = entity::countries::Entity::find()
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(countries)
}

pub async fn toggle_active(id: uuid::Uuid, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let exists = entity::countries::Entity::find_by_id(id)
        .filter(entity::countries::Column::IsDeleted.eq(false))
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Country not found".to_string())),
    };

    let mut model: entity::countries::ActiveModel = exists.into();

    if let ActiveValue::Set(is_active) = model.is_active {
        model.is_active = ActiveValue::Set(!is_active);
    }
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}

pub async fn toggle_deletion(id: uuid::Uuid, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let exists = entity::countries::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Country not found".to_string())),
    };

    let mut model: entity::countries::ActiveModel = exists.into();

    if let ActiveValue::Set(is_deleted) = model.is_deleted {
        model.is_deleted = ActiveValue::Set(!is_deleted);
    }
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}
