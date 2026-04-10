#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: String,
    pub subject_id: String,
    pub title: String,
    #[sqlx(skip)]
    pub questions: Vec<crate::Question>,
}

impl Chapter {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS chapter (
        id TEXT PRIMARY KEY,
        subject_id TEXT NOT NULL,
        title TEXT NOT NULL,
        FOREIGN KEY (subject_id) REFERENCES subject(id) ON DELETE CASCADE
    )"#;
    pub const INSERT_QUERY: &str =
        "INSERT INTO chapter (id, subject_id, title) VALUES (?1, ?2, ?3)";
}
