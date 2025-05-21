use actix_web::web;

use crate::{
    app::{
        events::{
            models::model::AddEventDto,
            services::service::{end_event, process_schedule_days},
        },
        schedules::dtos::dto::{get_non_recurring_schedules, get_schedules},
    },
    AppState,
};

pub async fn generate_event(state: &web::Data<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let today = chrono::Utc::now().naive_utc().date();

    let schedules = match get_schedules(state).await {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };

    for schedule in schedules {
        let start_date = schedule.start_date.unwrap_or(today);
        let end_date = schedule.end_date.unwrap_or(start_date);

        let data = AddEventDto {
            organization: schedule.organization_id,
            branch: schedule.branch_id,
            schedule: schedule.id,
            is_recurring: schedule.is_recurring,
        };

        process_schedule_days(start_date, end_date, data, state).await?;
    }

    Ok(())
}

pub async fn stop_non_recurring_event(
    state: &web::Data<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let today = chrono::Utc::now().naive_utc().date();

    let schedules = match get_non_recurring_schedules(state).await {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };

    for schedule in schedules {
        let end_date = schedule.end_date.unwrap_or(today);

        let data = AddEventDto {
            organization: schedule.organization_id,
            branch: schedule.branch_id,
            schedule: schedule.id,
            is_recurring: schedule.is_recurring,
        };

        end_event(data, end_date, state).await?;
    }

    Ok(())
}
