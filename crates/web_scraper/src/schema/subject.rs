use crate::error;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub key: String,
    pub exam_key: String,
    pub title: String,
}

impl Subject {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let mut output = Self::default();

        output.key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'key' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'key' field as a string".to_string(),
                )
            })?
            .to_string();
        output.exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output.title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'title' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'title' field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(output)
    }
}
