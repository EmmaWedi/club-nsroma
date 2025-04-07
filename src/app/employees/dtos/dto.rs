use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    ModelTrait, QueryFilter, Set,
};

use crate::{
    app::employees::models::model::{AddEmployeeDto, ApproveEmployeeDto, EmployeeResponse},
    libs::password::validate_password,
    utils::shared::{gen_num, parse_uuid},
    AppState,
};

pub async fn save_employee(
    data: AddEmployeeDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::employees::ActiveModel>, DbErr> {
    let employee = entity::employees::ActiveModel {
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        email: Set(data.email),
        contact: Set(data.contact),
        gender: Set(data.gender),
        date_of_birth: Set(data.date_of_birth),
        marital_status: Set(data.marital_status),
        branch_id: Set(data.branch),
        organization_id: Set(data.organization),
        department_id: Set(data.department),
        ..Default::default()
    };

    let result = entity::employees::Entity::insert(employee)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_employee_details(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<EmployeeResponse, DbErr> {
    let result = entity::employees::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()));

    let employee = result?;

    Ok(EmployeeResponse::from(employee))
}

pub async fn get_employee_session(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<EmployeeResponse, DbErr> {
    let result = entity::employees::Entity::find()
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::Session.eq(id.to_string()))
                .add(entity::employees::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()));

    let employee = result?;

    Ok(EmployeeResponse::from(employee))
}

pub async fn get_employee_with_auth(
    phone: String,
    ent_password: String,
    state: &web::Data<AppState>,
) -> Result<EmployeeResponse, DbErr> {
    let result = entity::employees::Entity::find()
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::Contact.eq(phone))
                .add(entity::employees::Column::IsBlocked.eq(false))
                .add(entity::employees::Column::IsApproved.eq(true)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()));

    let employee = result?;

    let salt = employee
        .salt
        .as_deref()
        .map(parse_uuid)
        .ok_or_else(|| DbErr::Custom("Salt is missing".into()))?;

    let hash = employee
        .password
        .as_deref()
        .ok_or_else(|| DbErr::Custom("Password is missing".into()))?;

    if validate_password(&ent_password, &salt, hash) {
        return Err(DbErr::Custom("Invalid credentials".to_string()));
    }

    let session = uuid::Uuid::new_v4();

    let mut model: entity::employees::ActiveModel = employee.into();

    model.session = ActiveValue::Set(Some(session.to_string()));
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    let updated = ActiveModelTrait::update(model, state.pg_db.get_ref()).await?;

    Ok(EmployeeResponse::from(updated))
}

pub async fn get_employee_details_comp(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<
    (
        EmployeeResponse,
        entity::organizations::Model,
        entity::branches::Model,
        entity::departments::Model,
    ),
    DbErr,
> {
    let (employee, organization) = entity::employees::Entity::find()
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::Session.eq(id.to_string()))
                .add(entity::employees::Column::IsBlocked.eq(false))
                .add(entity::employees::Column::IsApproved.eq(true)),
        )
        .find_also_related(entity::organizations::Entity)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()))?;

    let branch = employee
        .find_related(entity::branches::Entity)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()))?;

    let department = employee
        .find_related(entity::departments::Entity)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Department not found".into()))?;

    Ok((
        EmployeeResponse::from(employee),
        organization.ok_or_else(|| DbErr::RecordNotFound("Organization not found".into()))?,
        branch,
        department,
    ))
}

pub async fn approve_emp(
    data: ApproveEmployeeDto,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let result = entity::employees::Entity::find_by_id(data.id)
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::IsBlocked.eq(false))
                .add(entity::employees::Column::IsApproved.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()))?;

    let mut model: entity::employees::ActiveModel = result.into();

    let full_name = format!("{:?}{:?}", model.first_name, model.last_name);

    model.is_approved = ActiveValue::Set(true);
    model.approved_at = ActiveValue::Set(Some(chrono::Utc::now().into()));
    model.employee_number = ActiveValue::Set(Some(gen_num("EMP", full_name, &state).await?));
    model.password = ActiveValue::Set(Some(data.password));
    model.salt = ActiveValue::Set(Some(data.salt));
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}

pub async fn toggle_emp_block(id: uuid::Uuid, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let result = entity::employees::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::employees::Column::IsDeleted.eq(false))
                .add(entity::employees::Column::IsApproved.eq(true)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()))?;

    let mut model: entity::employees::ActiveModel = result.into();

    if let ActiveValue::Set(is_blocked) = model.is_blocked {
        model.is_blocked = ActiveValue::Set(!is_blocked);
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

pub async fn get_employee_by_contact(
    phone: String,
    state: &web::Data<AppState>,
) -> Result<EmployeeResponse, DbErr> {
    let result = entity::employees::Entity::find()
        .filter(entity::employees::Column::Contact.eq(phone))
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Employee not found".into()));

    let employee = result?;

    Ok(EmployeeResponse::from(employee))
}
