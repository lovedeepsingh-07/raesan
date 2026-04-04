#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Exam {
    pub id: String,
    pub title: String,
    #[sqlx(skip)]
    pub subjects: Vec<crate::Subject>,
}

impl Exam {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS exam (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL
    )"#;
    pub const INSERT_QUERY: &str = "INSERT INTO exam (id, title) VALUES (?1, ?2)";
    pub const LIST_QUERY: &str = "SELECT * FROM exam";
}
