pub mod constants;
pub mod examside;
pub mod utils;

pub use examside::ExamSide;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ScraperLog {
    Info(String),
    Warn(String),
}

pub trait Scraper {
    const BASE_URL: &str;
    fn scrape(
        db_pool: &sqlx::Pool<sqlx::Sqlite>,
        log_tx: mpsc::Sender<ScraperLog>,
    ) -> impl Future<Output = Result<(), error::Error>>;
}
