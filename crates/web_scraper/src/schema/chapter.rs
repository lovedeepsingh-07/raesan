use crate::{error, string_vec};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: String,
    pub key: String,
    pub exam_key: String,
    pub subject_id: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
}

impl Chapter {
    pub fn get_migration_queries() -> Vec<String> {
        string_vec![
            r#"CREATE TABLE IF NOT EXISTS chapter (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                title TEXT NOT NULL,
                "group" TEXT NOT NULL,
                FOREIGN KEY (subject_id) REFERENCES subject(id) ON DELETE CASCADE
            )"#
        ]
    }
    // JSON: {
    //   key: String,
    //   exam: String,
    //   subject: String,
    //   title: String,
    //   chapterGroup: String
    // }
    pub fn from_json(subject_id: String, json: &serde_json::Value) -> Result<Self, error::Error> {
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
            id: uuid::Uuid::new_v4().to_string(),
            key: chapter_key,
            exam_key,
            subject_id,
            subject_key,
            title: chapter_title,
            group: chapter_group,
        })
    }
}
