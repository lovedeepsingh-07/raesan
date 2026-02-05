mod error;
mod exam_page;
mod page_metadata;
pub mod schema;
pub mod tree_schema;

pub const JEE_MAIN_URL: &'static str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &'static str = "https://questions.examside.com/past-years/medical/neet";
pub const JEE_ADVANCED_URL: &'static str =
    "https://questions.examside.com/past-years/jee/jee-advanced";

pub async fn run() -> Result<(), error::Error> {
    let exam_page_metadata = page_metadata::extract(JEE_MAIN_URL).await?;
    let exam_data = exam_page::exam_data::extract(&exam_page_metadata)?;
    log::info!("{:#?}", exam_data);
    let (subject_store, chapter_json_array) =
        exam_page::subject_data::extract(&exam_page_metadata)?;
    log::info!("{:#?}", subject_store);
    let chapter_store = exam_page::chapter_data::extract(chapter_json_array)?;
    log::info!("{:#?}", chapter_store);
    Ok(())
}
