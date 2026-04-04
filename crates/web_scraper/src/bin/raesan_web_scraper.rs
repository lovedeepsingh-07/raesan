use sqlx::sqlite;
use std::str::FromStr;
use web_scraper::Scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    // create database connection
    let db_options =
        sqlite::SqliteConnectOptions::from_str("./test.db")?.create_if_missing(true);
    let db_pool = sqlite::SqlitePoolOptions::new()
        .connect_with(db_options)
        .await?;

    // run migrations
    for migration in schema::get_migration_queries() {
        sqlx::query(&migration).execute(&db_pool).await?;
    }

    match web_scraper::examside::ExamSide::scrape(&db_pool).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("{}", e);
        }
    };

    Ok(())
}
