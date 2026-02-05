#![allow(non_camel_case_types)]

use std::collections::HashMap;
use crate::schema;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Exam {
    pub key: String,
    pub title: String,
    pub group: String,
    pub subjects: HashMap<String, T_Subject>,
}
impl From<schema::Exam> for T_Exam {
    fn from(value: schema::Exam) -> Self {
        Self {
            key: value.key,
            title: value.title,
            group: value.group,
            subjects: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct T_Subject {
    pub key: String,
    pub exam_key: String,
    pub title: String,
    pub chapters: HashMap<String, T_Chapter>,
}
impl From<schema::Subject> for T_Subject {
    fn from(value: schema::Subject) -> Self {
        Self {
            key: value.key,
            exam_key: value.exam_key,
            title: value.title,
            chapters: HashMap::new(),
        }
    }
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
impl From<schema::Chapter> for T_Chapter {
    fn from(value: schema::Chapter) -> Self {
        Self {
            key: value.key,
            exam_key: value.exam_key,
            subject_key: value.subject_key,
            title: value.title,
            group: value.group,
            questions: Vec::new(),
        }
    }
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
impl From<schema::Question> for T_Question {
    fn from(value: schema::Question) -> Self {
        Self {
            question_type: T_QuestionType::from(value.question_type),
            exam_key: value.exam_key,
            subject_key: value.subject_key,
            chapter_key: value.chapter_key,
            chapter_group: value.chapter_group,
            content: value.content,
            options: value.options,
            answer: value.answer,
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum T_QuestionType {
    #[default]
    T_MCQ,
    T_INTEGER,
}
impl From<schema::QuestionType> for T_QuestionType {
    fn from(value: schema::QuestionType) -> Self {
        match value {
            schema::QuestionType::MCQ => T_QuestionType::T_MCQ,
            schema::QuestionType::INTEGER => T_QuestionType::T_INTEGER,
        }
    }
}
