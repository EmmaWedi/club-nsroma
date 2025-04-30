use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    ModelTrait, QueryFilter, Set,
};

use crate::{app::events::models::model::AddEventDto, AppState};

pub async fn create_event(
    data: AddEventDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::events::ActiveModel>, DbErr> {
    let event = entity::events::ActiveModel {
        organization_id: Set(data.organization),
        branch_id: Set(data.branch),
        schedule_id: Set(data.schedule),
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
        .filter(Condition::all().add(entity::events::Column::ScheduleId.eq(schedule)))
        .one(state.pg_db.get_ref())
        .await?
        .is_some();

    Ok(result)
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
