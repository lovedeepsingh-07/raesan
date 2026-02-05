use crate::error;

pub fn extract(page_metadata: &serde_json::Value) -> Result<(String, String), error::Error> {
    let chapter_metadata = page_metadata.get("chapter").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the chapter's 'chapter' field".to_string())
    })?;
    let subject_key = chapter_metadata
        .get("subject")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'subject' field".to_string(),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'subject' field as a string".to_string(),
            )
        })?
        .to_string();
    let chapter_key = chapter_metadata
        .get("key")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the chapter's 'key' field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'key' field as a string".to_string(),
            )
        })?
        .to_string();

    Ok((subject_key, chapter_key))
}
