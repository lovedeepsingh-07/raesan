#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: String,
    pub key: String,
    pub exam_key: String,
    pub subject_id: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
    #[sqlx(skip)]
    pub questions: Vec<crate::Question>,
}

impl Chapter {
    pub fn get_migration_queries() -> Vec<String> {
        vec![
            r#"CREATE TABLE IF NOT EXISTS chapter (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                title TEXT NOT NULL,
                "group" TEXT NOT NULL,
                FOREIGN KEY (subject_id) REFERENCES subject(id) ON DELETE CASCADE
            )"#
            .to_string(),
        ]
    }
}
