#![allow(non_camel_case_types)]

use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    pub key: String,
    pub title: String,
    pub group: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub key: String,
    pub exam_key: String,
    pub title: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub key: String,
    pub exam_key: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub question_type: QuestionType,
    pub exam_key: String,
    pub subject_key: String,
    pub chapter_key: String,
    pub chapter_group: String,
    pub content: String,
    pub options: HashMap<String, String>,
    pub answer: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum QuestionType {
    #[default]
    MCQ,
    INTEGER,
}
