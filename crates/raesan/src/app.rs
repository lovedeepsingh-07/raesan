use sqlx::sqlite;
use std::str::FromStr;

#[derive(Debug)]
pub struct App {
    pub env: crate::Environment,
    pub db_pool: sqlite::SqlitePool,
}

impl App {
    pub async fn new(db_path: &str, env: crate::Environment) -> Result<Self, error::Error> {
        let db_options = sqlite::SqliteConnectOptions::from_str(db_path)?.create_if_missing(false);
        let db_pool = sqlite::SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;

        Ok(Self { env, db_pool })
    }
}
