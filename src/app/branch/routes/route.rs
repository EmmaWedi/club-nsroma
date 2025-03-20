use actix_web::web;

use crate::{
    app::branch::controllers::controller::add_branch,
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/branches").route(
            "/add",
            web::post()
                .to(add_branch)
                .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                .wrap(JwtAuthMiddleware),
        ),
    );
}
