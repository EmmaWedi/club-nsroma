use actix_web::web;

use crate::{
    app::users::controllers::controller::{get_user_details, signin_user},
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/users")
            .route("/signin", web::post().to(signin_user))
            .route(
                "/details",
                web::get()
                    .to(get_user_details)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
