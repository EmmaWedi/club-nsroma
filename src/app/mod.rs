use actix_web::web::{self, ServiceConfig};

use crate::AppState;

pub fn app_routes(state: web::Data<AppState>) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut web::ServiceConfig| {
        cfg.configure(health::routes::route::route);
        cfg.configure(files_manager::file_api::route);
        cfg.configure(|c| country::routes::route::routes(c, state.clone()));
        cfg.configure(|c| organization::routes::route::routes(c, state.clone()));
        cfg.configure(|c| users::routes::route::routes(c, state.clone()));
    }
}

pub mod country;
pub mod files_manager;
pub mod health;
pub mod organization;
pub mod branch;
pub mod employees;
pub mod users;
