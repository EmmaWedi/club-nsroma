use actix_web::web;

use crate::{
    app::departments::controllers::controller::{
        add_department, get_department_detail, get_departments_by_branch,
        get_departments_by_organization, toggle_deletion,
    },
    middlewares::{auth::JwtAuthMiddleware, checker::CheckUserMiddleware},
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/departments")
            .route(
                "/add",
                web::post()
                    .to(add_department)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/organizations/get",
                web::get()
                    .to(get_departments_by_organization)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/branches/{id}",
                web::get()
                    .to(get_departments_by_branch)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/get/{id}",
                web::get()
                    .to(get_department_detail)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            )
            .route(
                "/delete/{id}",
                web::delete()
                    .to(toggle_deletion)
                    .wrap(CheckUserMiddleware::new(state.clone(), "User"))
                    .wrap(JwtAuthMiddleware),
            ),
    );
}
