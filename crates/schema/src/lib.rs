pub mod chapter;
pub mod exam;
pub mod question;
pub mod question_option;
pub mod question_type;
pub mod source_record;
pub mod subject;

pub use chapter::Chapter;
pub use exam::Exam;
pub use question::Question;
pub use question_option::QuestionOption;
pub use question_type::QuestionType;
pub use source_record::{EntityType, ScraperType, SourceRecord};
pub use subject::Subject;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct RaesanTest_ChapterSummary {
    pub chapter_id: String,
    pub chapter_name: String,
    pub subject_name: String,
    pub exam_name: String,
    pub question_count: i32,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RaesanTest {
    pub id: String,
    pub chapter_summaries: Vec<RaesanTest_ChapterSummary>,
    pub created_at: i64,
    pub total_questions: usize,
    pub total_mcq_questions: usize,
    pub total_integer_questions: usize,
    pub questions: Vec<Question>,
}

pub fn get_migration_queries() -> Vec<&'static str> {
    vec![
        "PRAGMA foreign_keys = ON;",
        Exam::MIGRATION_QUERY,
        Subject::MIGRATION_QUERY,
        Chapter::MIGRATION_QUERY,
        Question::MIGRATION_QUERY,
        QuestionOption::MIGRATION_QUERY,
        SourceRecord::MIGRATION_QUERY,
    ]
}
