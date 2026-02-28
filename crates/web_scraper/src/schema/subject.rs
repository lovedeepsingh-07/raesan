use crate::error;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub key: String,
    pub exam_key: String,
    pub title: String,
}

impl Subject {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let subject_key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[key] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[key] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[exam] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[exam] field as a string".to_string(),
                )
            })?
            .to_string();
        let subject_title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[title] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[title] field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(Subject {
            key: subject_key,
            exam_key,
            title: subject_title,
        })
    }
}
