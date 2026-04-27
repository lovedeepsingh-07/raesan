#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct QuestionOption {
    pub id: String,
    pub question_id: String,
    pub key: String,
    pub value: String,
}

impl QuestionOption {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS question_option (
            id TEXT PRIMARY KEY,
            question_id TEXT NOT NULL,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (question_id) REFERENCES question(id) ON DELETE CASCADE,
            UNIQUE (question_id, key)
        )"#;
    // above, the "UNIQUE (question_id, key)" means the key must be unique per
    // question_id, this ensures there are no duplicate keys like two A, for 1 question
    pub const INSERT_QUERY: &str =
        "INSERT INTO question_option (id, question_id, key, value) VALUES (?1, ?2, ?3, ?4)";
    pub const SELECT_QUERY: &str = "SELECT * FROM question_option";
}
