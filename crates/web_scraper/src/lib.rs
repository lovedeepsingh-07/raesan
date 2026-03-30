pub mod examside;
pub mod utils;

pub use examside::ExamSide;

#[derive(Debug)]
pub enum AvailableScrapers {
    ExamSide,
}

pub trait Scraper {
    const BASE_URL: &str;
    fn scrape() -> impl Future<Output = Result<Vec<schema::Exam>, error::Error>>;
}
