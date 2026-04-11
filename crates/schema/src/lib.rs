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
