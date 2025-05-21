use actix_web::web;
use migration::Expr;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::{app::events::models::model::AddEventDto, AppState};

pub async fn create_event(
    data: AddEventDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::events::ActiveModel>, DbErr> {
    let event = entity::events::ActiveModel {
        organization_id: Set(data.organization),
        branch_id: Set(data.branch),
        schedule_id: Set(data.schedule),
        is_recurring: Set(Some(data.is_recurring)),
        ..Default::default()
    };

    let result = entity::events::Entity::insert(event)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_event_by_schedule(
    schedule: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<bool, DbErr> {
    let result = entity::events::Entity::find()
        .filter(
            Condition::all()
                .add(entity::events::Column::ScheduleId.eq(schedule))
                .add(entity::events::Column::IsActive.eq(true))
                .add(entity::events::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .is_some();

    Ok(result)
}

pub async fn get_schedule_event(
    schedule: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::events::Model, DbErr> {
    let result = entity::events::Entity::find()
        .filter(
            Condition::all()
                .add(entity::events::Column::ScheduleId.eq(schedule))
                .add(entity::events::Column::IsActive.eq(true))
                .add(entity::events::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Event not found".into()));

    result
}

pub async fn get_event_by_id(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::events::Model, DbErr> {
    let result = entity::events::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Event not found".into()));

    result
}

pub async fn toggle_activeness(
    id: uuid::Uuid,
    schedule: uuid::Uuid,
    branch: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let event = entity::events::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::events::Column::Id.eq(id))
                .add(entity::events::Column::BranchId.eq(branch))
                .add(entity::events::Column::ScheduleId.eq(schedule))
                .add(entity::events::Column::IsDeleted.eq(false)),
        )
        .col_expr(
            entity::events::Column::IsActive,
            Expr::col(entity::events::Column::IsActive).not(),
        )
        .col_expr(
            entity::events::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    if event.rows_affected == 0 {
        return Err(DbErr::Custom("Event not found".to_string()));
    };

    Ok(())
}

pub async fn toggle_deletion(
    id: uuid::Uuid,
    schedule: uuid::Uuid,
    branch: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let event = entity::events::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::events::Column::Id.eq(id))
                .add(entity::events::Column::BranchId.eq(branch))
                .add(entity::events::Column::ScheduleId.eq(schedule))
                .add(entity::events::Column::IsActive.eq(false)),
        )
        .col_expr(
            entity::events::Column::IsDeleted,
            Expr::col(entity::events::Column::IsDeleted).not(),
        )
        .col_expr(
            entity::events::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    if event.rows_affected == 0 {
        return Err(DbErr::Custom("Event not found".to_string()));
    };

    Ok(())
}

pub async fn activeness_job(
    schedule: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let event = entity::events::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::events::Column::ScheduleId.eq(schedule))
                .add(entity::events::Column::IsDeleted.eq(false)),
        )
        .col_expr(
            entity::events::Column::IsActive,
            Expr::col(entity::events::Column::IsActive).not(),
        )
        .col_expr(
            entity::events::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    if event.rows_affected == 0 {
        return Err(DbErr::Custom("Event not found".to_string()));
    };

    Ok(())
}
