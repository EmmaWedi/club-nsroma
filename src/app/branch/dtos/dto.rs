use actix_web::web;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::{app::branch::models::model::AddBranchDto, AppState};

pub async fn save_branch(
    data: AddBranchDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::branches::ActiveModel>, DbErr> {
    let branch = entity::branches::ActiveModel {
        name: Set(data.name),
        organization_id: Set(data.organization),
        location: Set(Some(data.location)),
        post_code: Set(Some(data.gps)),
        country_id: Set(data.country),
        contact: Set(Some(data.contact)),
        email: Set(data.email),
        ..Default::default()
    };

    let result = entity::branches::Entity::insert(branch)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_active_branches(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::branches::Model>, DbErr> {
    let branches = entity::branches::Entity::find()
        .filter(
            Condition::all()
                .add(entity::branches::Column::IsActive.eq(true))
                .add(entity::branches::Column::IsDeleted.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(branches)
}

pub async fn get_branch(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::branches::Model, DbErr> {
    let branch = entity::branches::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::branches::Column::IsActive.eq(true))
                .add(entity::branches::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()));

    branch
}

pub async fn get_organization_branches(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::branches::Model>, DbErr> {
    let branches = entity::branches::Entity::find()
        .filter(
            Condition::all()
                .add(entity::branches::Column::OrganizationId.eq(id))
                .add(entity::branches::Column::IsActive.eq(true))
                .add(entity::branches::Column::IsDeleted.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(branches)
}
