pub mod chapter_metadata;
pub mod question_array;
pub mod question;

use crate::{error, schema};

pub async fn extract(
    page_metadata: serde_json::Value,
    exam_schema: &mut schema::Exam,
) -> Result<(), error::Error> {
    let (subject_key, chapter_key) = chapter_metadata::extract(&page_metadata)?;
    let subject_schema: &mut schema::Subject =
        exam_schema.subjects.get_mut(&subject_key).ok_or_else(|| {
            error::Error::NotFoundError(format!("No subject by the key {:#?} found", subject_key))
        })?;
    let chapter_schema: &mut schema::Chapter = subject_schema
        .chapters
        .get_mut(&chapter_key)
        .ok_or_else(|| {
            error::Error::NotFoundError(format!("No chapter by the key {:#?} found", chapter_key))
        })?;
    question_array::extract(page_metadata, chapter_schema)?;
    Ok(())
}
