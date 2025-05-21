use std::sync::Arc;

use actix_cors::Cors;
use actix_http::header::{self};
use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use app::app_routes;
use config::{Config as ConfigLoader, ConfigError, File, FileFormat};
use jobs::launchjobs;
use libs::error;
use middlewares::helmet::SecurityHeaders;
use sea_orm::DatabaseConnection;
use setup::db::pg::pg_conn;

mod app;
mod jobs;
mod libs;
mod mailer;
mod middlewares;
mod setup;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub config: ConfigLoader,
    pub pg_db: Arc<Data<DatabaseConnection>>,
}

fn load_config() -> Result<ConfigLoader, ConfigError> {
    let config = ConfigLoader::builder()
        .set_default("default", "1")?
        .add_source(File::new("src/app.config", FileFormat::Toml))
        .build()?;

    Ok(config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    use tokio::signal;

    env_logger::init();

    let settings = load_config().expect("Failed to load configuration");

    let port = settings.get::<String>("app.port").unwrap();
    let host = settings.get::<String>("app.host").unwrap();

    let pg_conn: Arc<Data<DatabaseConnection>> = Arc::new(Data::new(pg_conn(&settings).await));

    let _port = port.clone();
    let _host = host.clone();

    let state = web::Data::new(AppState {
        config: settings.clone(),
        pg_db: pg_conn.clone(),
    });

    let job_state = state.clone();

    tokio::spawn(async move {
        launchjobs(job_state).await;
    });

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        log::info!("==> 🚀 listening at {}:{}", host, port);

        App::new()
            .app_data(state.clone())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::METHOD_NOT_ALLOWED, error::render_405)
                    .handler(http::StatusCode::NOT_FOUND, error::render_404)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, error::render_500)
                    .handler(http::StatusCode::BAD_REQUEST, error::render_400),
            )
            .wrap(SecurityHeaders)
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(cors)
            .configure(app_routes(state.clone()))
    })
    .bind(format!("{}:{}", _host, _port))?
    .run();

    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
        log::info!("Shutdown signal received.");
    };

    tokio::select! {
        _ = server => {
            log::info!("HTTP server exited.");
        }
        _ = shutdown_signal => {
            log::info!("Initiating graceful shutdown...");
        }
    }

    Ok(())
}
