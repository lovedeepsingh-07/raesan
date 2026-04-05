use sqlx::sqlite;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub app_env: Environment,
    pub db_pool: sqlite::SqlitePool,
}

impl ServerState {
    pub async fn new(db_path: &str) -> Result<Self, error::Error> {
        let app_env = match std::env::var("PUBLIC_APP_ENV") {
            Ok(out) => match out.as_str() {
                "production" => Environment::PROD,
                _ => Environment::DEV,
            },
            Err(e) => {
                log::warn!("Failed to get APP_ENV, assuming DEV, error: {}", e);
                Environment::DEV
            }
        };
        let db_options = sqlite::SqliteConnectOptions::from_str(db_path)?.create_if_missing(false);
        let db_pool = sqlite::SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;

        Ok(Self { app_env, db_pool })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    DEV,
    PROD,
}
