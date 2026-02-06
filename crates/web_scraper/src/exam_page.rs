use crate::{error, schema};

pub fn extract(
    exam_page_metadata: &serde_json::Value,
) -> Result<(schema::Exam, Vec<schema::Subject>, Vec<schema::Chapter>), error::Error> {
    let exam = schema::Exam::from_json(exam_page_metadata)?;

    let mut subject_store: Vec<schema::Subject> = Vec::new();
    let subjects_field = exam_page_metadata.get("subjects").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field".to_string())
    })?;
    let subjects_array = subjects_field.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field as an array".to_string())
    })?;
    let mut chapter_json_array: Vec<&serde_json::Value> = Vec::new();

    for subject_json in subjects_array {
        let subject = schema::Subject::from_json(&subject_json)?;
        subject_store.push(subject);

        let chapters_field = subject_json.get("chapters").ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the 'chapters' field".to_string())
        })?;
        let chapters_array = chapters_field.as_array().ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the 'chapters' field as an array".to_string(),
            )
        })?;
        chapter_json_array.extend(chapters_array);
    }

    let mut chapter_store: Vec<schema::Chapter> = Vec::new();

    for chapter_json in chapter_json_array {
        let chapter = schema::Chapter::from_json(&chapter_json)?;
        chapter_store.push(chapter);
    }

    Ok((exam, subject_store, chapter_store))
}
