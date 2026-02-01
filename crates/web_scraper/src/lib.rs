mod error;
mod exam_page;
mod page_metadata;
mod schema;

pub const JEE_MAIN_URL: &'static str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &'static str = "https://questions.examside.com/past-years/medical/neet";
pub const JEE_ADVANCED_URL: &'static str =
    "https://questions.examside.com/past-years/jee/jee-advanced";

pub async fn run() -> Result<(), error::Error> {
    let exam_page_metadata = page_metadata::extract(JEE_MAIN_URL).await?;
    exam_page::extract(exam_page_metadata).await?;
    Ok(())
}
