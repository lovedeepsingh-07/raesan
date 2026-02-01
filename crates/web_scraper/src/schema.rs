use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    pub key: String,
    pub title: String,
    pub group: String,
    pub subjects: HashMap<String, Subject>
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub key: String,
    pub title: String,
    pub chapters: HashMap<String, Chapter>
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub key: String,
    pub exam_key: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
}
