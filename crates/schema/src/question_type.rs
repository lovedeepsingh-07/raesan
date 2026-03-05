#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum QuestionType {
    #[default]
    MCQ,
    INTEGER,
}

impl From<&str> for QuestionType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mcq" => QuestionType::MCQ,
            _ => QuestionType::INTEGER,
        }
    }
}
impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::MCQ => write!(f, "mcq"),
            QuestionType::INTEGER => write!(f, "integer"),
        }
    }
}
