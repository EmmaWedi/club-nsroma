use config::Config;
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn pg_conn(settings: &Config) -> DatabaseConnection {
    dotenv().unwrap();

    let idle_timeout = settings.get::<u64>("pg.idle_timeout").unwrap();
    let max = settings.get::<u32>("pg.max").unwrap();

    let db_env_check = settings.get::<String>("app.environment").unwrap();

    let database_url: String;

    match db_env_check.as_str() {
        "TEST" => {
            database_url = std::env::var("DATABASE_URL").unwrap();
        }
        _ => {
            database_url = std::env::var("DATABASE_URL").unwrap();
        }
    }

    let mut options = ConnectOptions::new(database_url);

    options
        .max_connections(max)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::new(idle_timeout, 0))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(true) //for prod change this to false
        .sqlx_logging_level(log::LevelFilter::Info); //for prod change the Info to Off

    let pool = Database::connect(options).await.unwrap();

    pool
}
