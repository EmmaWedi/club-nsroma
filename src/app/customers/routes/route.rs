use actix_web::web;

use crate::{
    app::customers::controllers::controller::{
        get_details, signin_customer, signup, update_details,
    },
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/customers")
            .route("/signin", web::post().to(signin_customer))
            .route("/signup", web::post().to(signup))
            .route(
                "/details",
                web::get()
                    .to(get_details)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Customer"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/update",
                web::put()
                    .to(update_details)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Customer"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
