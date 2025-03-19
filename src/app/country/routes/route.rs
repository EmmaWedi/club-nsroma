use actix_web::web;

use crate::{
    app::country::controllers::controller::{
        add_country, get_active_countries, get_country_details, toggle_activeness, toggle_delete,
    },
    AppState,
};

pub fn routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/countries")
            .route("/add", web::post().to(add_country))
            .route("/get", web::get().to(get_active_countries))
            .route("/active", web::get().to(toggle_activeness))
            .route("/remove", web::delete().to(toggle_delete))
            .route("/single/{id}", web::get().to(get_country_details)),
    );
}
