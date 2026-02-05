use crate::{error, schema};

pub fn extract(exam_page_metadata: &serde_json::Value) -> Result<schema::Exam, error::Error> {
    let mut output = schema::Exam::default();

    output.key = exam_page_metadata
        .get("key")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam's key' field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam's 'key' field as a string".to_string(),
            )
        })?
        .to_string();
    output.title = exam_page_metadata
        .get("title")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam's 'title' field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam's 'title' field as a string".to_string(),
            )
        })?
        .to_string();
    output.group = exam_page_metadata
        .get("examGroup")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam's 'examGroup' field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam's 'examGroup' field as a string".to_string(),
            )
        })?
        .to_string();

    Ok(output)
}
