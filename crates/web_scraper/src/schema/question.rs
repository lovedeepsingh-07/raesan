use crate::{error, schema, string_vec};
use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct QuestionOption {
    pub id: String,
    pub question_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub id: String,
    pub exam_key: String,
    pub subject_key: String,
    pub chapter_id: String,
    pub chapter_key: String,
    pub chapter_group: String,
    pub question_type: schema::QuestionType,
    pub content: String,
    pub options: HashMap<String, QuestionOption>,
    pub answer: String,
}

impl Question {
    pub fn get_migration_queries() -> Vec<String> {
        string_vec![
            r#"CREATE TABLE IF NOT EXISTS question (
                id TEXT PRIMARY KEY,
                chapter_id TEXT NOT NULL,
                question_type TEXT NOT NULL,
                content TEXT NOT NULL,
                answer TEXT NOT NULL,
                FOREIGN KEY (chapter_id) REFERENCES chapter(id) ON DELETE CASCADE
            )"#,
            r#"CREATE TABLE IF NOT EXISTS question_option (
                id TEXT PRIMARY KEY,
                question_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                FOREIGN KEY (question_id) REFERENCES question(id) ON DELETE CASCADE,
                UNIQUE (question_id, key)
            )"#
        ]
        // NOTE: above, the "UNIQUE (question_id, key)" means the key must be unique per
        // question_id, this ensures there are no duplicate keys like two A, for 1 question
    }
    // JSON: {
    //   exam: String,
    //   subject: String,
    //   chapter: String,
    //   chapterGroup: String
    //   question: {
    //     en: {
    //       content: String,
    //       correct_options: [String],
    //       answer: String,
    //       options: [ { identifier: String, content: String } ]
    //     }
    //   }
    // }
    pub fn from_json(
        chapter_id: String,
        question_type: schema::QuestionType,
        json: &serde_json::Value,
    ) -> Result<Self, error::Error> {
        let exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the question[exam] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[exam] field as a string".to_string(),
                )
            })?
            .to_string();
        let subject_key = json
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[subject] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[subject] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_key = json
            .get("chapter")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapter] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapter] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_group = json
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapterGroup] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapterGroup] field as a string".to_string(),
                )
            })?
            .to_string();

        let question_body_data = json
            .get("question")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question] field".to_string(),
                )
            })?
            .get("en")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en] field".to_string(),
                )
            })?;
        let question_content = question_body_data
            .get("content")
            .ok_or_else(|| {
                error::Error::DeserializeError(format!(
                    "Failed to get the question[question][en][content] field, {:#?}",
                    question_body_data
                ))
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][content] field as a string"
                        .to_string(),
                )
            })?
            .to_string();

        let question_id = uuid::Uuid::new_v4().to_string();
        let question_answer = question_type.get_answer(question_body_data)?;
        let question_options =
            question_type.get_options(question_body_data, question_id.clone())?;
        Ok(Question {
            id: question_id,
            exam_key,
            subject_key,
            chapter_key,
            chapter_id,
            chapter_group,
            question_type,
            content: question_content,
            options: question_options,
            answer: question_answer,
        })
    }
}
