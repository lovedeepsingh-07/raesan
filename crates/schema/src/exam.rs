#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Exam {
    pub id: String,
    pub key: String,
    pub title: String,
    pub group: String,
    #[sqlx(skip)]
    pub subjects: Vec<crate::Subject>
}

impl Exam {
    pub fn get_migration_queries() -> Vec<String> {
        vec![
            r#"CREATE TABLE IF NOT EXISTS exam (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                title TEXT NOT NULL,
                "group" TEXT NOT NULL
            )"#
            .to_string(),
        ]
    }
}
