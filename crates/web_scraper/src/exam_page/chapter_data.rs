use crate::{error, schema};

pub fn extract<'a>(
    chapter_json_array: Vec<&'a serde_json::Value>,
) -> Result<schema::ChapterStore, error::Error> {
    let mut output = schema::ChapterStore::default();

    for chapter_json in chapter_json_array {
        let mut output_chapter = schema::Chapter::default();

        output_chapter.key = chapter_json
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
        output_chapter.exam_key = chapter_json
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
        output_chapter.subject_key = chapter_json
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
        output_chapter.title = chapter_json
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
        output_chapter.group = chapter_json
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

        output.0.push(output_chapter);
    }

    Ok(output)
}
