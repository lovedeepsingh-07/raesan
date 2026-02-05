#![allow(non_camel_case_types)]

use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Exam {
    pub key: String,
    pub title: String,
    pub group: String,
    pub subjects: HashMap<String, T_Subject>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Subject {
    pub key: String,
    pub title: String,
    pub chapters: HashMap<String, T_Chapter>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Chapter {
    pub key: String,
    pub exam_key: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
    pub questions: Vec<T_Question>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Question {
    pub question_type: T_QuestionType,
    pub exam_key: String,
    pub subject_key: String,
    pub chapter_key: String,
    pub chapter_group: String,
    pub content: String,
    pub options: HashMap<String, String>,
    pub answer: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum T_QuestionType {
    #[default]
    T_MCQ,
    T_INTEGER,
}
