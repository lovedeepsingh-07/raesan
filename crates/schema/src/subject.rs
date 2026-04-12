#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: String,
    pub exam_id: String,
    pub title: String,
    pub total_chapters: i64,
    #[sqlx(skip)]
    pub chapters: Vec<crate::Chapter>,
}

impl Subject {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS subject (
        id TEXT PRIMARY KEY,
        exam_id TEXT NOT NULL,
        title TEXT NOT NULL,
        total_chapters INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (exam_id) REFERENCES exam(id) ON DELETE CASCADE
    )"#;
    pub const INSERT_QUERY: &str = "INSERT INTO subject (id, exam_id, title) VALUES (?1, ?2, ?3)";
}
