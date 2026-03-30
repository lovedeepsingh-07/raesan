use crate::examside::{ChapterFromJson, ExamFromJson, SubjectFromJson};

pub fn extract(
    exam_page_metadata: &serde_json::Value,
) -> Result<schema::Exam, error::Error> {
    let mut exam = schema::Exam::from_json(exam_page_metadata)?;

    let subjects_array = exam_page_metadata
        .get("subjects")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam[subjects] field".to_string())
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam[subjects] field as an array".to_string(),
            )
        })?;

    for subject_json in subjects_array {
        let mut subject = schema::Subject::from_json(exam.id.clone(), subject_json)?;

        let chapters_array = subject_json
            .get("chapters")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapters] field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapters] field as an array".to_string(),
                )
            })?;
        for chapter_json in chapters_array {
            let chapter = schema::Chapter::from_json(subject.id.clone(), chapter_json)?;
            subject.chapters.push(chapter);
        }

        exam.subjects.push(subject);
    }

    Ok(exam)
}
