#![allow(non_camel_case_types)]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Root(pub serde_json::Value, pub ExamWrapper);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExamWrapper {
   pub data: ExamData
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExamData {
    pub key: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub subjects: Vec<SubjectData>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubjectData {
   pub key: String,
   pub name: Option<String>,
   pub title: Option<String>,
   pub chapters: Vec<ChapterData>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChapterData {
   pub key: String,
   #[serde(rename = "title")]
   pub name: String,
   #[serde(rename = "chapterGroup")]
   pub group: String,
}
