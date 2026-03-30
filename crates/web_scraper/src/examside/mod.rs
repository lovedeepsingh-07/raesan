pub mod from_json;
pub mod pages;
pub mod identifier;

pub use from_json::ChapterFromJson;
pub use from_json::ExamFromJson;
pub use from_json::QuestionFromJson;
pub use from_json::QuestionTypeFromJson;
pub use from_json::SubjectFromJson;

#[derive(Debug)]
pub struct ExamSide {}

impl ExamSide {
    pub const FETCH_PATHS: [&str; 3] = [
        "past-years/jee/jee-main",
        "past-years/medical/neet",
        "past-years/jee/jee-advanced",
    ];
}

impl crate::Scraper for ExamSide {
    const BASE_URL: &str = "https://questions.examside.com";
    async fn scrape() -> Result<Vec<schema::Exam>, error::Error> {
        let mut output: Vec<schema::Exam> = Vec::new();

        for curr_path in Self::FETCH_PATHS {
            let exam_page_metadata = pages::metadata::extract(&format!("{}/{}", Self::BASE_URL, curr_path)).await?;
            let exam = pages::exam_page::extract(&exam_page_metadata)?;
            output.push(exam);
        }

        Ok(output)
    }
}
