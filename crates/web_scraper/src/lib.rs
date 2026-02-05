mod chapter_page;
pub mod error;
mod exam_page;
mod page_metadata;
pub mod schema;
pub mod tree_schema;

#[derive(Debug, Default)]
pub struct WebScraper {
    exam_store: Vec<schema::Exam>,
    subject_store: Vec<schema::Subject>,
    chapter_store: Vec<schema::Chapter>,
    question_store: Vec<schema::Question>
}

impl WebScraper {
    pub async fn extract(&mut self, exam_url: &str) -> Result<(), error::Error> {
        let exam_page_metadata = page_metadata::extract(exam_url).await?;

        let exam = exam_page::exam_data::extract(&exam_page_metadata)?;
        self.exam_store.push(exam);

        let (subject_store, chapter_json_array) = exam_page::subject_data::extract(&exam_page_metadata)?;
        self.subject_store.extend(subject_store);

        let chapter_store = exam_page::chapter_data::extract(chapter_json_array)?;
        self.chapter_store.extend(chapter_store);

        let chapter_page_metadata = page_metadata::extract(format!("{}/{}/{}", exam_url, self.chapter_store[0].subject_key, self.chapter_store[0].key).as_str()).await?;
        self.question_store.extend(chapter_page::extract(&chapter_page_metadata)?);

        Ok(())
    }
}
