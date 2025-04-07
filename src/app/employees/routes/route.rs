use actix_web::web;

use crate::{
    app::employees::controllers::controller::{
        add_employee, approve_employee, block_employee, emp_details, signin_employee,
    },
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/employees")
            .route("/signin", web::post().to(signin_employee))
            .route(
                "/details",
                web::get()
                    .to(emp_details)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/add",
                web::post()
                    .to(add_employee)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/approve/{id}",
                web::get()
                    .to(approve_employee)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/block/{id}",
                web::get()
                    .to(block_employee)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
