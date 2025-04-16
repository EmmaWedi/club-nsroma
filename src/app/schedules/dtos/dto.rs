use actix_web::web;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{
    app::schedules::models::model::{AddScheduleDto, ToggleDiscountDto, ToggleRecurringDto},
    AppState,
};

pub async fn save_schedule(
    data: AddScheduleDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::schedules::ActiveModel>, DbErr> {
    let schedule = entity::schedules::ActiveModel {
        organization_id: Set(data.organization),
        branch_id: Set(data.branch),
        description: Set(Some(data.description)),
        name: Set(data.name),
        fee: Set(data.fee),
        ..Default::default()
    };

    let result = entity::schedules::Entity::insert(schedule)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_schedules(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::schedules::Model>, DbErr> {
    let schedules = entity::schedules::Entity::find()
        .filter(
            Condition::all()
                .add(entity::schedules::Column::IsDeleted.eq(false))
                .add(entity::schedules::Column::IsActive.eq(true)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(schedules)
}

pub async fn get_schedules_by_org(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::schedules::Model>, DbErr> {
    let schedules = entity::schedules::Entity::find()
        .filter(
            Condition::all()
                .add(entity::schedules::Column::IsDeleted.eq(false))
                .add(entity::schedules::Column::OrganizationId.eq(id)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(schedules)
}

pub async fn get_schedules_by_branch(
    id: uuid::Uuid,
    branch: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::schedules::Model>, DbErr> {
    let schedules = entity::schedules::Entity::find()
        .filter(
            Condition::all()
                .add(entity::schedules::Column::IsDeleted.eq(false))
                .add(entity::schedules::Column::OrganizationId.eq(id))
                .add(entity::schedules::Column::BranchId.eq(branch)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(schedules)
}

pub async fn get_schedule(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::schedules::Model, DbErr> {
    let schedule = entity::schedules::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::schedules::Column::IsDeleted.eq(false))
                .add(entity::schedules::Column::IsActive.eq(true)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Schedule not found".into()));

    schedule
}

pub async fn toggle_recurring(
    id: uuid::Uuid,
    data: ToggleRecurringDto,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let schedule = entity::schedules::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::schedules::Column::BranchId.eq(data.branch))
                .add(entity::schedules::Column::OrganizationId.eq(data.organization))
                .add(entity::schedules::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Schedule not found".into()))?;

    let mut model: entity::schedules::ActiveModel = schedule.into();

    if let ActiveValue::Set(is_recurring) = model.is_recurring {
        let new_is_recurring = !is_recurring;
        model.is_recurring = ActiveValue::Set(new_is_recurring);

        if !new_is_recurring {
            model.start_date = ActiveValue::Set(data.start_date);
            model.end_date = ActiveValue::Set(data.end_date);
        };
    };

    if let Some(recurring_type) = data.recurring_type {
        model.recurring_type = ActiveValue::Set(recurring_type.to_string());
    }

    model.start_time = ActiveValue::Set(Some(data.start_time));
    model.end_time = ActiveValue::Set(Some(data.end_time));
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}

pub async fn toggle_discount(
    id: uuid::Uuid,
    data: ToggleDiscountDto,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let schedule = entity::schedules::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::schedules::Column::BranchId.eq(data.branch))
                .add(entity::schedules::Column::OrganizationId.eq(data.organization))
                .add(entity::schedules::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Schedule not found".into()))?;

    let mut model: entity::schedules::ActiveModel = schedule.into();

    if let ActiveValue::Set(is_discounted) = model.is_discounted {
        let new_is_discounted = !is_discounted;
        model.is_discounted = ActiveValue::Set(new_is_discounted);

        if new_is_discounted {
            if let Some(rate) = data.rate {
                model.discount_rate = ActiveValue::Set(rate);
            };
        }
    };

    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}

pub async fn toggle_activation(
    id: uuid::Uuid,
    branch: uuid::Uuid,
    organization: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let schedule = entity::schedules::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::schedules::Column::Id.eq(id))
                .add(entity::schedules::Column::BranchId.eq(branch))
                .add(entity::schedules::Column::OrganizationId.eq(organization))
                .add(entity::schedules::Column::IsDeleted.eq(false)),
        )
        // .col_expr(entity::schedules::Column::IsActive, Expr::value(true))
        .col_expr(
            entity::schedules::Column::IsActive,
            Expr::col(entity::schedules::Column::IsActive).not(), //this would toggle this field
        )
        .col_expr(
            entity::schedules::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    if schedule.rows_affected == 0 {
        return Err(DbErr::Custom("Schedule not found".to_string()));
    }

    Ok(())
}

pub async fn set_student_schedule(
    id: uuid::Uuid,
    branch: uuid::Uuid,
    organization: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let schedule = entity::schedules::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::schedules::Column::Id.eq(id))
                .add(entity::schedules::Column::BranchId.eq(branch))
                .add(entity::schedules::Column::OrganizationId.eq(organization))
                .add(entity::schedules::Column::IsDeleted.eq(false)),
        )
        .col_expr(
            entity::schedules::Column::IsStudentEvent,
            Expr::col(entity::schedules::Column::IsStudentEvent).not(),
        )
        .col_expr(
            entity::schedules::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    if schedule.rows_affected == 0 {
        return Err(DbErr::Custom("Schedule not found".to_string()));
    };

    Ok(())
}
