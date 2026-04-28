#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum EntityType {
    #[default]
    Exam,
    Subject,
    Chapter,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum ScraperType {
    #[default]
    ExamSide,
}

// on different sites, there are different ways to identify a specifc record like a specific
// exam or a specific chapter etc, on 1 site they might be using a slug for this, but on some other
// site they might be using a UUID for the same thing
// The "SourceRecord" struct handles this possibility by connecting the record (exam, subject, etc)
// with the "scraper_type" and uses "source_key" to represent the key by which the record can be
// identified on a website
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct SourceRecord {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub scraper_type: ScraperType,
    pub source_key: String,
}
impl SourceRecord {
    pub const MIGRATION_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS source_record (
        entity_id TEXT PRIMARY KEY,
        entity_type TEXT NOT NULL,
        scraper_type TEXT NOT NULL,
        source_key TEXT NOT NULL
    )"#;
    pub const INSERT_QUERY: &str = "INSERT INTO source_record (entity_id, entity_type, scraper_type, source_key) VALUES (?1, ?2, ?3, ?4)";
}
