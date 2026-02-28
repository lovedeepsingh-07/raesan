use crate::{error, schema};

pub fn extract(
    exam_page_metadata: &serde_json::Value,
) -> Result<(schema::Exam, Vec<schema::Subject>, Vec<schema::Chapter>), error::Error> {
    let exam = schema::Exam::from_json(exam_page_metadata)?;

    let mut subject_store: Vec<schema::Subject> = Vec::new();
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

    // NOTE: instead of going through each subject in a loop and then going through each chapter in
    // a sub loop, possible creating a O(n^2) situation, I just go through each subject and get the
    // chapters for that subject and store them in a separate vector so that after I am done with
    // the subjects, I can just go through chapters in a separate loop
    let mut chapter_json_array: Vec<&serde_json::Value> = Vec::new();
    for subject_json in subjects_array {
        let subject = schema::Subject::from_json(subject_json)?;
        subject_store.push(subject);

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
        chapter_json_array.extend(chapters_array);
    }

    let mut chapter_store: Vec<schema::Chapter> = Vec::new();
    for chapter_json in chapter_json_array {
        let chapter = schema::Chapter::from_json(chapter_json)?;
        chapter_store.push(chapter);
    }

    Ok((exam, subject_store, chapter_store))
}
