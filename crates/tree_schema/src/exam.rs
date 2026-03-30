#![allow(non_camel_case_types)]

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct T_Exam {
    pub id: String,
    pub key: String,
    pub title: String,
    pub group: String,
    #[sqlx(skip)]
    pub subjects: Vec<crate::T_Subject>,
}

impl From<&schema::Exam> for T_Exam {
    fn from(value: &schema::Exam) -> Self {
        Self {
            id: value.id.clone(),
            key: value.key.clone(),
            title: value.title.clone(),
            group: value.group.clone(),
            subjects: Vec::new(),
        }
    }
}

impl T_Exam {
    pub const LIST_QUERY: &str = "SELECT * FROM exam";
}
