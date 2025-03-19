use actix_web::web;

use crate::{
    app::organization::controllers::controller::{
        add_organization, get_active_organizations, get_organization_details,
    },
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/organizations")
            .route("/add", web::post().to(add_organization))
            .route(
                "/get/{id}",
                web::get()
                    .to(get_organization_details)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/get",
                web::get()
                    .to(get_active_organizations)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
