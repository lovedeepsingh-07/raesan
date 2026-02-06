use crate::{error, schema};
use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub question_type: schema::QuestionType,
    pub exam_key: String,
    pub subject_key: String,
    pub chapter_key: String,
    pub chapter_group: String,
    pub content: String,
    pub options: HashMap<String, String>,
    pub answer: String,
}

impl Question {
    pub fn from_json(
        question_type: schema::QuestionType,
        json: &serde_json::Value,
    ) -> Result<Self, error::Error> {
        let mut output = Self::default();

        output.exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output.subject_key = json
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'subject' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'subject' field as a string".to_string(),
                )
            })?
            .to_string();
        output.chapter_key = json
            .get("chapter")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'chapter' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'chapter' field as a string".to_string(),
                )
            })?
            .to_string();
        output.chapter_group = json
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'chapterGroup' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'chapterGroup' field as a string".to_string(),
                )
            })?
            .to_string();

        let question_body_data = json
            .get("question")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'question' field".to_string(),
                )
            })?
            .get("en")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'question' field".to_string(),
                )
            })?;
        output.content = question_body_data
            .get("content")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    // "Failed to get the question's 'content' field".to_string(),
                    format!(
                        "Failed to get the question's 'content' field, {:#?}",
                        question_body_data
                    ),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'content' field as a string".to_string(),
                )
            })?
            .to_string();

        output.answer = question_type.get_answer(&question_body_data)?;
        output.options = question_type.get_options(&question_body_data)?;
        Ok(output)
    }
}
