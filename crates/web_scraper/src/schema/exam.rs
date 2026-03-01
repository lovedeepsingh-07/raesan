use crate::{error, string_vec};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Exam {
    pub id: String,
    pub key: String,
    pub title: String,
    pub group: String,
}

impl Exam {
    pub fn get_migration_queries() -> Vec<String> {
        string_vec![
            r#"CREATE TABLE IF NOT EXISTS exam (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                title TEXT NOT NULL,
                "group" TEXT NOT NULL
            )"#
        ]
    }
    // JSON: { key: String, tite: String, examGroup: String }
    pub fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let exam_key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the exam[key] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[key] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the exam[title] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[title] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_group = json
            .get("examGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[examGroup] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[examGroup] field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(Exam {
            id: uuid::Uuid::new_v4().to_string(),
            key: exam_key,
            title: exam_title,
            group: exam_group,
        })
    }
}
