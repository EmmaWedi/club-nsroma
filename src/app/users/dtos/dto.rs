use actix_web::web;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::{
    app::users::models::model::{AddUserDto, UserResponse},
    libs::{jwt::create_jwt, password::validate_password},
    utils::shared::parse_uuid,
    AppState,
};

pub async fn save_users(
    data: AddUserDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::users::ActiveModel>, DbErr> {
    let user = entity::users::ActiveModel {
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        organization_id: Set(data.organization),
        contact: Set(data.contact),
        email: Set(data.email),
        session: Set(Some(data.session)),
        salt: Set(Some(data.salt)),
        password: Set(Some(data.password)),
        ..Default::default()
    };

    let result = entity::users::Entity::insert(user)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_users(state: &web::Data<AppState>) -> Result<Vec<UserResponse>, DbErr> {
    let results = entity::users::Entity::find()
        .filter(entity::users::Column::IsBlocked.eq(false))
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    let users: Vec<UserResponse> = results.into_iter().map(UserResponse::from).collect();

    Ok(users)
}

pub async fn get_user(id: uuid::Uuid, state: &web::Data<AppState>) -> Result<UserResponse, DbErr> {
    let result = entity::users::Entity::find_by_id(id)
        .filter(entity::users::Column::IsBlocked.eq(false))
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".into()));

    let user = result?;
    Ok(UserResponse::from(user))
}

pub async fn get_user_session(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<UserResponse, DbErr> {
    let result = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::IsBlocked.eq(false))
                .add(entity::users::Column::Session.eq(id.to_string())),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".into()));

    let user = result?;
    Ok(UserResponse::from(user))
}

pub async fn get_user_with_auth(
    phone: String,
    ent_password: String,
    state: &web::Data<AppState>,
) -> Result<UserResponse, DbErr> {
    let result = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Contact.eq(phone))
                .add(entity::users::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".into()));

    let user = result?;

    let salt = user
        .salt
        .as_deref()
        .map(parse_uuid)
        .ok_or_else(|| DbErr::Custom("Salt is missing".to_string()))?;

    let hash = user
        .password
        .as_deref()
        .ok_or_else(|| DbErr::Custom("Password is missing".into()))?;

    if !validate_password(&ent_password, &salt, hash) {
        return Err(DbErr::Custom("Invalid credentials".to_string()));
    }

    let session = uuid::Uuid::new_v4();

    let mut model: entity::users::ActiveModel = user.into();

    model.session = ActiveValue::Set(Some(session.to_string()));
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    let updated_user = ActiveModelTrait::update(model, state.pg_db.get_ref()).await?;

    Ok(UserResponse::from(updated_user))
}

pub async fn save_user_with_token(
    data: AddUserDto,
    state: &web::Data<AppState>,
) -> Result<String, DbErr> {
    let user = entity::users::ActiveModel {
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        organization_id: Set(data.organization),
        contact: Set(data.contact),
        email: Set(data.email),
        session: Set(Some(data.session)),
        salt: Set(Some(data.salt)),
        password: Set(Some(data.password)),
        ..Default::default()
    };

    let result = entity::users::Entity::insert(user)
        .exec_with_returning(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    let session = result
        .session
        .as_deref()
        .ok_or_else(|| DbErr::Custom("Session is missing".into()))?;

    let gen_token = create_jwt(session.to_string(), &state).await;

    Ok(gen_token.token)
}
