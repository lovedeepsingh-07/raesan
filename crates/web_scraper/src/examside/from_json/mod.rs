use std::collections::HashMap;

pub mod chapter;
pub mod exam;
pub mod question;
pub mod question_type;
pub mod subject;

pub trait ExamFromJson {
    fn from_json(json: &serde_json::Value) -> Result<Self, error::Error>
    where
        Self: Sized;
}
pub trait SubjectFromJson {
    fn from_json(exam_id: String, json: &serde_json::Value) -> Result<Self, error::Error>
    where
        Self: Sized;
}
pub trait ChapterFromJson {
    fn from_json(subject_id: String, json: &serde_json::Value) -> Result<Self, error::Error>
    where
        Self: Sized;
}
pub trait QuestionFromJson {
    fn from_json(
        chapter_id: String,
        question_type: schema::QuestionType,
        json: &serde_json::Value,
    ) -> Result<Self, error::Error>
    where
        Self: Sized;
}
pub trait QuestionTypeFromJson {
    fn get_answer<'a>(&'a self, json: &'a serde_json::Value) -> Result<String, error::Error>;
    fn get_options<'a>(
        &'a self,
        question_id: String,
        json: &'a serde_json::Value,
    ) -> Result<HashMap<String, schema::QuestionOption>, error::Error>
    where
        Self: Sized;
}
