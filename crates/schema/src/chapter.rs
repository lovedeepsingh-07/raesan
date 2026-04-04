#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: String,
    #[sqlx(skip)]
    pub key: String,
    #[sqlx(skip)]
    pub exam_key: String,
    pub subject_id: String,
    #[sqlx(skip)]
    pub subject_key: String,
    pub title: String,
    #[sqlx(skip)]
    pub group: String,
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
    pub const LIST_QUERY: &str = "SELECT * FROM chapter";
}
