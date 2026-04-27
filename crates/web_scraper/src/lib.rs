pub mod constants;
pub mod examside;
pub mod utils;

pub use examside::ExamSide;
use tokio::sync::mpsc;

// rexport some external crates in order to ease the pain of someone trying to this as a library
pub use sqlx;
pub use tokio;

#[derive(Debug, Clone)]
pub enum ScraperLog {
    Info(String),
    Warn(String),
}

// you can just implement this trait for a struct to implement a "Scraper" for a specific
// website, it contains a single "scrape" function that takes in three arguments:
//
// - "base_url": it is the home url of the website you are trying to scrape, if you are implementing
// this trait for a website which you can scrape legally, no issue but if you are by chancing
// trying to "sail the high seas", this kind of helps you minimize the connection between the
// website you are trying to scrape and your scraper, ideally you should have the base_url be
// included via an environment variable so it is difficult to figure out which website you are
// actually scraping, atleast that's just what I think, maybe it's completely useless, who knows
//
// - "db_pool": sqlx sqlite database connection pool
//
// - "log_tx": I didn't want the scraper's to do literally anything other than scrape, so even the
// logging is delegated, so you just supply a "Sender" of a "tokio::mpsc" channel and the scraper
// sends logs to that channel, you can use the "Receiver" of that channel to do whatever you want
// with the logs
pub trait Scraper {
    fn scrape(
        base_url: &str,
        db_pool: &sqlx::Pool<sqlx::Sqlite>,
        log_tx: mpsc::Sender<ScraperLog>,
    ) -> impl Future<Output = Result<(), error::Error>>;
}
