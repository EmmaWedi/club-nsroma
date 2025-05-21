use actix_web::web;

use crate::{
    app::events::{
        dtos::dto::{activeness_job, create_event, get_event_by_schedule},
        models::model::AddEventDto,
    },
    AppState,
};

pub async fn process_schedule_days(
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    data: AddEventDto,
    state: &web::Data<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current = start_date;

    let schedule = data.schedule;

    while current <= end_date {
        let exists = match get_event_by_schedule(schedule, state).await {
            Ok(exists) => exists,
            Err(e) => return Err(Box::new(e)),
        };

        if !exists {
            create_event_for_schedule(data.clone(), state).await?;
        }

        current += chrono::Duration::days(1);
    }

    Ok(())
}

async fn create_event_for_schedule(
    data: AddEventDto,
    state: &web::Data<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = AddEventDto {
        organization: data.organization,
        branch: data.branch,
        schedule: data.schedule,
        is_recurring: data.is_recurring,
    };

    let _ = create_event(data, state)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);

    Ok(())
}

pub async fn end_event(
    data: AddEventDto,
    end_date: chrono::NaiveDate,
    state: &web::Data<AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let today = chrono::Utc::now().naive_utc().date();

    if !data.is_recurring && today > end_date {
        let _ = activeness_job(data.schedule, state)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
    }

    Ok(())
}
