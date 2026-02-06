use crate::error;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    pub key: String,
    pub title: String,
    pub group: String,
}

impl Exam {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let mut output = Self::default();

        output.key = json
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
        output.title = json
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
        output.group = json
            .get("examGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam's 'examGroup' field".to_string(),
                )
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
}
