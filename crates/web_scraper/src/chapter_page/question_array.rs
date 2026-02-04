use crate::{chapter_page::question, error, schema};

pub fn extract(
    page_metadata: serde_json::Value,
    chapter_schema: &mut schema::Chapter,
) -> Result<(), error::Error> {
    let question_types_array = page_metadata
        .get("questions")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'question' field".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'question' field as an array".to_string(),
            )
        })?;

    for question_type in question_types_array {
        question::extract(question_type, chapter_schema)?;
    }
    log::info!("{:#?}", chapter_schema);
    Ok(())
}
