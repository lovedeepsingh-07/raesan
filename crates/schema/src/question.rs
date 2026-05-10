#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: String,
    pub chapter_id: String,
    pub question_type: crate::QuestionType,
    pub content: String,
    #[sqlx(skip)]
    pub options: Vec<crate::QuestionOption>,
    pub answer: String,
}

impl Question {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS question (
            id TEXT PRIMARY KEY,
            chapter_id TEXT NOT NULL,
            question_type TEXT NOT NULL,
            content TEXT NOT NULL,
            answer TEXT NOT NULL,
            FOREIGN KEY (chapter_id) REFERENCES chapter(id) ON DELETE CASCADE
        )"#;
    pub const INSERT_QUERY: &str = "INSERT INTO question (id, chapter_id, question_type, content, answer) VALUES (?1, ?2, ?3, ?4, ?5)";
}
