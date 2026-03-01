use crate::{error, string_vec};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: String,
    pub key: String,
    pub exam_id: String,
    pub exam_key: String,
    pub title: String,
}

impl Subject {
    pub fn get_migration_queries() -> Vec<String> {
        string_vec![
            r#"CREATE TABLE IF NOT EXISTS subject (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                exam_id TEXT NOT NULL,
                title TEXT NOT NULL,
                FOREIGN KEY (exam_id) REFERENCES exam(id) ON DELETE CASCADE
            )"#
        ]
    }
    // JSON: { key: String, exam: String, title: String }
    pub fn from_json(exam_id: String, json: &serde_json::Value) -> Result<Self, error::Error> {
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
            id: uuid::Uuid::new_v4().to_string(),
            key: subject_key,
            exam_key,
            exam_id,
            title: subject_title,
        })
    }
}
