use crate::error;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub key: String,
    pub exam_key: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
}

impl Chapter {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let mut output = Self::default();

        output.key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'key' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'key' field as a string".to_string(),
                )
            })?
            .to_string();
        output.exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output.subject_key = json
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'subject' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'subject' field as a string".to_string(),
                )
            })?
            .to_string();
        output.title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'title' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'title' field as a string".to_string(),
                )
            })?
            .to_string();
        output.group = json
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'chapterGroup' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'chapterGroup' field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(output)
    }
}
