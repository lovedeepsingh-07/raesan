#![allow(non_camel_case_types)]

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct T_Subject {
    pub id: String,
    pub key: String,
    pub exam_id: String,
    pub exam_key: String,
    pub title: String,
    #[sqlx(skip)]
    pub chapters: Vec<crate::T_Chapter>,
}

impl From<&schema::Subject> for T_Subject {
    fn from(value: &schema::Subject) -> Self {
        Self {
            id: value.id.clone(),
            key: value.key.clone(),
            exam_id: value.exam_id.clone(),
            exam_key: value.exam_key.clone(),
            title: value.title.clone(),
            chapters: Vec::new(),
        }
    }
}
