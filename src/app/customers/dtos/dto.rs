use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{
    app::customers::models::model::{AddCustomerDto, CustomerResponse, UpdateCustomerDto},
    apply_update_wrap,
    libs::password::validate_password,
    utils::shared::parse_uuid,
    AppState,
};

pub async fn save_customer(
    data: AddCustomerDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::customers::ActiveModel>, DbErr> {
    let customer = entity::customers::ActiveModel {
        username: Set(Some(data.username)),
        contact: Set(data.contact),
        customer_number: Set(data.customer_number),
        password: Set(Some(data.password)),
        salt: Set(Some(data.salt)),
        ..Default::default()
    };

    let result = entity::customers::Entity::insert(customer)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_customer_details(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<CustomerResponse, DbErr> {
    let customer = entity::customers::Entity::find()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Session.eq(id.to_string()))
                .add(entity::customers::Column::IsBlocked.eq(false))
                .add(entity::customers::Column::IsDeleted.eq(false))
                .add(entity::customers::Column::IsActive.eq(true)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    Ok(CustomerResponse::from(customer))
}

pub async fn get_customer_with_auth(
    phone: String,
    ent_password: String,
    state: &web::Data<AppState>,
) -> Result<CustomerResponse, DbErr> {
    let customer = entity::customers::Entity::find()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Contact.eq(phone))
                .add(entity::customers::Column::IsBlocked.eq(false))
                .add(entity::customers::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    let salt = customer
        .salt
        .as_deref()
        .map(parse_uuid)
        .ok_or_else(|| DbErr::Custom("Salt is missing".into()))?;

    let hash = customer
        .password
        .as_deref()
        .ok_or_else(|| DbErr::Custom("Password is missing".into()))?;

    if validate_password(&ent_password, &salt, hash) {
        return Err(DbErr::Custom("Invalid credentials".to_string()));
    }

    let session = uuid::Uuid::new_v4();

    let mut model: entity::customers::ActiveModel = customer.into();

    model.session = ActiveValue::Set(Some(session.to_string()));
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    let updated = ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(CustomerResponse::from(updated))
}

pub async fn update_customer_details(
    id: uuid::Uuid,
    data: UpdateCustomerDto,
    state: &web::Data<AppState>,
) -> Result<CustomerResponse, DbErr> {
    let customer = entity::customers::Entity::find()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Session.eq(id.to_string()))
                .add(entity::customers::Column::IsBlocked.eq(false))
                .add(entity::customers::Column::IsActive.eq(true))
                .add(entity::customers::Column::IsDeleted.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    let mut model: entity::customers::ActiveModel = customer.into();

    apply_update_wrap!(model, data,
        email: email => Some,
        username: username => Some
    );

    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    let updated = ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(CustomerResponse::from(updated))
}

