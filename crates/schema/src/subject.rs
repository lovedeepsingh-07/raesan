#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: String,
    pub key: String,
    pub exam_id: String,
    pub exam_key: String,
    pub title: String,
    #[sqlx(skip)]
    pub chapters: Vec<crate::Chapter>
}

impl Subject {
    pub fn get_migration_queries() -> Vec<String> {
        vec![
            r#"CREATE TABLE IF NOT EXISTS subject (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                exam_id TEXT NOT NULL,
                title TEXT NOT NULL,
                FOREIGN KEY (exam_id) REFERENCES exam(id) ON DELETE CASCADE
            )"#
            .to_string(),
        ]
    }
}
