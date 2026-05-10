use sqlx::sqlite;
use std::str::FromStr;
use tokio::sync::mpsc;
use web_scraper::Scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let db_options = sqlite::SqliteConnectOptions::from_str(
        format!("./{}.db", raesan::constants::DB_NAME).as_str(),
    )?
    .create_if_missing(true);
    let db_pool = sqlite::SqlitePoolOptions::new()
        .connect_with(db_options)
        .await?;

    for migration in schema::get_migration_queries() {
        sqlx::query(migration).execute(&db_pool).await?;
    }

    let examside_base_url = match std::env::var(raesan::environment::EXAMSIDE_BASE_URL__NAME) {
        Ok(out) => out,
        Err(e) => {
            log::error!(
                "Failed to get {:#?} environment variable, {}",
                raesan::environment::EXAMSIDE_BASE_URL__NAME,
                e
            );
            return Ok(());
        }
    };

    let (log_tx, mut log_rx) = mpsc::channel::<web_scraper::ScraperLog>(64);
    let scraper_db_pool = db_pool.clone();
    let scraper_handle = tokio::spawn(async move {
        web_scraper::examside::ExamSide::scrape(
            examside_base_url.as_str(),
            &scraper_db_pool,
            log_tx,
        )
        .await
    });

    while let Some(scraper_log) = log_rx.recv().await {
        match scraper_log {
            web_scraper::ScraperLog::Info(msg) => {
                log::info!("{}", msg);
            }
            web_scraper::ScraperLog::Warn(msg) => {
                log::warn!("{}", msg);
            }
        }
    }

    match scraper_handle.await? {
        Ok(_) => {}
        Err(e) => log::error!("{}", e),
    }

    Ok(())
}
