#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Exam {
    pub id: String,
    pub title: String,
    pub total_subjects: i64,
    #[sqlx(skip)]
    pub subjects: Vec<crate::Subject>,
}

impl Exam {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS exam (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        total_subjects INTEGER NOT NULL DEFAULT 0
    )"#;
    pub const INSERT_QUERY: &str = "INSERT INTO exam (id, title) VALUES (?1, ?2)";
}
