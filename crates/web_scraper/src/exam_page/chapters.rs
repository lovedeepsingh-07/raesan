use crate::{error, schema};

pub async fn extract(
    subject_data: &serde_json::Value,
    output: &mut schema::Subject,
) -> Result<(), error::Error> {
    let chapters_field = subject_data.get("chapters").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'chapters' field".to_string())
    })?;
    let chapters_array = chapters_field.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'chapters' field as an array".to_string())
    })?;

    for chapter in chapters_array {
        let mut output_chapter = schema::Chapter::default();

        output_chapter.key = chapter
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'key' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'key' field as a string".to_string(),
                )
            })?
            .to_string();
        output_chapter.exam_key = chapter
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output_chapter.subject_key = chapter
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'subject' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'subject' field as a string".to_string(),
                )
            })?
            .to_string();
        output_chapter.title = chapter
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'title' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'title' field as a string".to_string(),
                )
            })?
            .to_string();
        output_chapter.group = chapter
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapter's 'chapterGroup' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the chapters' 'chapterGroup' field as a string".to_string(),
                )
            })?
            .to_string();

        output
            .chapters
            .insert(output_chapter.key.clone(), output_chapter);
    }
    Ok(())
}
