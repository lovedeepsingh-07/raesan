#![allow(non_camel_case_types)]

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct T_Chapter {
    pub id: String,
    pub key: String,
    pub exam_key: String,
    pub subject_id: String,
    pub subject_key: String,
    pub title: String,
    pub group: String,
    #[sqlx(skip)]
    pub questions: Vec<schema::Question>,
}

impl From<&schema::Chapter> for T_Chapter {
    fn from(value: &schema::Chapter) -> Self {
        Self {
            id: value.id.clone(),
            key: value.key.clone(),
            exam_key: value.exam_key.clone(),
            subject_id: value.subject_id.clone(),
            subject_key: value.subject_key.clone(),
            title: value.title.clone(),
            group: value.group.clone(),
            questions: Vec::new(),
        }
    }
}
