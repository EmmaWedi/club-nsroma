use actix_web::web;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::{app::organization::models::model::AddOrganizationDto, AppState};

pub async fn save_organization(
    data: AddOrganizationDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::organizations::ActiveModel>, DbErr> {
    let today = chrono::Utc::now().date_naive();

    let organization = entity::organizations::ActiveModel {
        name: Set(data.name),
        registered_at: Set(today
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .single()
            .unwrap()
            .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())),
        location: Set(Some(data.location)),
        post_code: Set(Some(data.post_code)),
        is_blocked: Set(false),
        is_active: Set(true),
        country_id: Set(data.country),
        ..Default::default()
    };

    let result = entity::organizations::Entity::insert(organization)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(result)
}

pub async fn get_organizations(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::organizations::Model>, DbErr> {
    let organizations = entity::organizations::Entity::find()
        .filter(
            Condition::all()
                .add(entity::organizations::Column::IsActive.eq(true))
                .add(entity::organizations::Column::IsBlocked.eq(false)),
        )
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(organizations)
}

pub async fn get_organization(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::organizations::Model, DbErr> {
    let organization = entity::organizations::Entity::find_by_id(id)
        .filter(
            Condition::all()
                .add(entity::organizations::Column::IsActive.eq(true))
                .add(entity::organizations::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found".into()));

    organization
}
