use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{app::departments::models::model::AddDepartmentDto, AppState};

pub async fn save_department(
    data: AddDepartmentDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::departments::ActiveModel>, DbErr> {
    let department = entity::departments::ActiveModel {
        name: Set(data.name),
        organization_id: Set(data.organization),
        branch_id: Set(data.branch),
        description: Set(Some(data.description)),
        is_for_all_branches: Set(data.for_all),
        number_of_employees: Set(Some(data.employee_num)),
        number_of_allowed_leave_days: Set(Some(data.leave_days)),
        daily_rate: Set(data.daily_rate),
        hourly_rate: Set(data.hourly_rate),
        ..Default::default()
    };

    let result = entity::departments::Entity::insert(department)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_departments(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::departments::Model>, DbErr> {
    let departments = entity::departments::Entity::find()
        .filter(entity::departments::Column::IsDeleted.eq(false))
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(departments)
}

pub async fn get_department(
    id: uuid::Uuid,
    organization: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::departments::Model, DbErr> {
    let department = entity::departments::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::departments::Column::OrganizationId.eq(organization))
                .add(entity::departments::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Department not found".into()));

    department
}

pub async fn get_organization_departments(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::departments::Model>, DbErr> {
    let departments = entity::departments::Entity::find()
        .filter(
            Condition::all()
                .add(entity::departments::Column::OrganizationId.eq(id))
                .add(entity::departments::Column::IsDeleted.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(departments)
}

pub async fn get_branch_departments(
    id: uuid::Uuid,
    branch: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::departments::Model>, DbErr> {
    let departments = entity::departments::Entity::find()
        .filter(
            Condition::all()
                .add(entity::departments::Column::OrganizationId.eq(id))
                .add(entity::departments::Column::BranchId.eq(branch))
                .add(entity::departments::Column::IsDeleted.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(departments)
}

pub async fn toggle_delete(
    id: uuid::Uuid,
    organization: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let exists = entity::departments::Entity::find_by_id(id)
        .filter(entity::departments::Column::OrganizationId.eq(organization))
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Department not found".into())),
    };

    let mut model: entity::departments::ActiveModel = exists.into();

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
