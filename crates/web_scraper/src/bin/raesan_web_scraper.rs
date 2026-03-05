use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let options = SqliteConnectOptions::from_str("sqlite://test.db")?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    // Run migrations
    let mut conn = pool.acquire().await?;
    for migration in schema::get_migration_queries() {
        sqlx::query(&migration).execute(&mut *conn).await?;
    }
    drop(conn);

    let mut tx = pool.begin().await?;
    let ron_data: Vec<tree_schema::T_Exam> = ron::from_str(&std::fs::read_to_string("test.ron")?)?;

    tx.commit().await?;

    Ok(())
}
