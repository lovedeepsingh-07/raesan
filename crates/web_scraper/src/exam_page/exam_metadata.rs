use crate::{error, schema};

pub async fn extract(data: &serde_json::Value, output: &mut schema::Exam) -> Result<(), error::Error> {
    output.key = data
        .get("key")
        .ok_or_else(|| error::Error::DeserializeError("Failed to get the exam's key' field".to_string()))?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam's 'key' field as a string".to_string())
        })?
        .to_string();
    output.title = data
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
    output.group = data
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
    Ok(())
}
