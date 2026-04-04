pub mod examside;
pub mod utils;

pub use examside::ExamSide;

pub trait Scraper {
    const BASE_URL: &str;
    fn scrape(db_pool: &sqlx::Pool<sqlx::Sqlite>)
    -> impl Future<Output = Result<(), error::Error>>;
}
