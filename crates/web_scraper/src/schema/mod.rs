pub mod chapter;
pub mod exam;
pub mod question;
pub mod question_type;
pub mod subject;

pub use chapter::Chapter;
pub use exam::Exam;
pub use question::Question;
pub use question::QuestionOption;
pub use question_type::QuestionType;
pub use subject::Subject;

pub fn get_migration_queries() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    out.push("PRAGMA foreign_keys = ON;".to_string());
    out.extend(Exam::get_migration_queries());
    out.extend(Subject::get_migration_queries());
    out.extend(Chapter::get_migration_queries());
    out.extend(Question::get_migration_queries());

    out
}
