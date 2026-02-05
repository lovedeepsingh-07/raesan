mod chapter_page;
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
    let _ = exam_page::exam_data::extract(&exam_page_metadata)?;
    let (_, chapter_json_array) =
        exam_page::subject_data::extract(&exam_page_metadata)?;
    let chapter_store = exam_page::chapter_data::extract(chapter_json_array)?;

    let chapter_page_metadata = page_metadata::extract(format!("{}/{}/{}", JEE_MAIN_URL, chapter_store.0[0].subject_key, chapter_store.0[0].key).as_str()).await?;
    let question_store = chapter_page::extract(&chapter_page_metadata);
    log::info!("{:#?}", question_store);
    Ok(())
}
