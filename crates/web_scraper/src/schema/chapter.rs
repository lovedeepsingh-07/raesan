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
        let chapter_key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the chapter[key] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters[key] field as a string".to_string(),
                )
            })?
            .to_string();

        let exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the chapter[exam] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters[exam] field as a string".to_string(),
                )
            })?
            .to_string();
        let subject_key = json
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter[subject] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters[subject] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the chapter[title] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters[title] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_group = json
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter[chapterGroup] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters[chapterGroup] field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(Chapter {
            key: chapter_key,
            exam_key,
            subject_key,
            title: chapter_title,
            group: chapter_group,
        })
    }
}
