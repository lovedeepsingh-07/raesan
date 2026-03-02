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
    pub question_type: crate::QuestionType,
    pub content: String,
    pub options: HashMap<String, QuestionOption>,
    pub answer: String,
}

impl Question {
    pub fn get_migration_queries() -> Vec<String> {
        vec![
            r#"CREATE TABLE IF NOT EXISTS question (
                id TEXT PRIMARY KEY,
                chapter_id TEXT NOT NULL,
                question_type TEXT NOT NULL,
                content TEXT NOT NULL,
                answer TEXT NOT NULL,
                FOREIGN KEY (chapter_id) REFERENCES chapter(id) ON DELETE CASCADE
            )"#
            .to_string(),
            r#"CREATE TABLE IF NOT EXISTS question_option (
                id TEXT PRIMARY KEY,
                question_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                FOREIGN KEY (question_id) REFERENCES question(id) ON DELETE CASCADE,
                UNIQUE (question_id, key)
            )"#
            .to_string(),
        ]
        // NOTE: above, the "UNIQUE (question_id, key)" means the key must be unique per
        // question_id, this ensures there are no duplicate keys like two A, for 1 question
    }
}
