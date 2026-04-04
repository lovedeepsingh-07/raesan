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
    let mut out: Vec<&'static str> = Vec::new();
    out.push("PRAGMA foreign_keys = ON;");
    out.push(Exam::MIGRATION_QUERY);
    out.push(Subject::MIGRATION_QUERY);
    out.push(Chapter::MIGRATION_QUERY);
    out.push(Question::MIGRATION_QUERY);
    out.push(QuestionOption::MIGRATION_QUERY);
    out.push(SourceRecord::MIGRATION_QUERY);
    out
}
