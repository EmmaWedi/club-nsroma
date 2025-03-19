use actix_web::web;

use crate::{app::users::controllers::controller::signin_user, AppState};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/users")
        .route("/signin", web::post().to(signin_user))
    );
}