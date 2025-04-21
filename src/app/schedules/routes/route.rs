use actix_web::web;

use crate::{
    app::schedules::controllers::controller::{
        create_schedule, get_active_schedules, get_branch_schedules, get_organization_schedules,
        get_schedule_details, set_student, tog_occurance, upd_discount,
    },
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/schedules")
            .route(
                "/add",
                web::post()
                    .to(create_schedule)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            )
            .route("/active", web::get().to(get_active_schedules))
            .route("/org/{id}", web::get().to(get_organization_schedules))
            .route(
                "/branch",
                web::get()
                    .to(get_branch_schedules)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            )
            .route("/details/{id}", web::get().to(get_schedule_details))
            .route(
                "/reoccur/{id}",
                web::put()
                    .to(tog_occurance)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/discount/{id}",
                web::put()
                    .to(upd_discount)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/toggle/{id}/student/{branch}",
                web::put()
                    .to(set_student)
                    .wrap(CheckUserMiddleware::new(state.clone(), "Employee"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
